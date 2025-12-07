#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

mod data;
use data::MIDI_DATA;

mod sound_profiles;
use sound_profiles::{INSTRUMENTS, SoundProfile};

use esp_backtrace as _;

use esp_hal::{
    analog::dac::Dac,
    clock::CpuClock,
    gpio::{AnyPin, Input, InputConfig, Level, Output, OutputConfig, Pin, Pull},
    main,
};

use esp_println::println;
use log::info;

use heapless::{Deque, LinearMap, Vec};

use midly::{
    EventIter, Header, MetaMessage, MidiMessage, Timing, TrackEventKind,
    num::{u4, u7},
    parse,
};

// =============================================================================================
//                         WRITE REGISTERS FOR PIN 0 - 31 FOR FAST TOGGLING
// =============================================================================================

const GPIO_0_31_SET_REG: *mut u32 = 0x3FF44008 as *mut u32; // set bit
const GPIO_0_31_CLEAR_REG: *mut u32 = 0x3FF4400C as *mut u32; // clear bit

// =============================================================================================
//                                              TIMER
// =============================================================================================

// esp_hal has timers and delays, but they were 1 micro second accuracy at best, while I need tunable ~50 nano second accuracy
// not as portable as esp_hal delay, but definitely more accurate

#[inline(always)]
pub fn delay_cycles(cycles: u32) {
    let start = read_ccount();
    while read_ccount().wrapping_sub(start) < cycles {
        core::hint::spin_loop();
    }
}

#[inline(always)]
fn read_ccount() -> u32 {
    let count: u32;
    unsafe {
        core::arch::asm!("rsr.ccount {0}", out(reg) count);
    }
    count
}

// =============================================================================================
//                                      SONG METADATA
// =============================================================================================

#[derive(Debug, Clone, Copy)]
struct SongMetaData {
    ticks_per_quarter: u16,  // ticks per quarter note
    tempo: u32,              // micro seconds per quarter note
    _bpm: u16,               // ms / min = 60_000_000, so BPM = 60_000_000 / tempo
    time_signature: [u8; 4], // [beats per measure, denominator of the time signature as 1/2^n,midi clock per quarter note, Number of Notated 32nd Notes in a MIDI Quarter Note]
    key: (i8, bool),         // ((-n_of_flats, + n_of_sharps), major / minor)
}

impl SongMetaData {
    fn new(header: Header) -> Self {
        let timing = match header.timing {
            Timing::Metrical(metric_timing) => metric_timing.as_int(),
            Timing::Timecode(_, _) => unimplemented!(),
        };

        Self {
            ticks_per_quarter: timing,
            tempo: 500_000,                // default tempo
            _bpm: 120,                     // default BPM
            time_signature: [4, 4, 24, 8], // default: 4/4
            key: (0, false),               // default: C major
        }
    }

    fn refresh_bpm(&mut self, tempo: u32) {
        const MICROS_PER_MIN: u32 = 60_000_000;
        self._bpm = (MICROS_PER_MIN / tempo) as u16;
    }
}

// =============================================================================================
//                                      SONG PLAYER
// =============================================================================================

struct SongPlayer<'a> {
    instrument_sounds: [SoundProfile; 16],
    free_buzzers: Deque<SoundBuzzer<'a>, 16>,
    taken_buzzers: LinearMap<(u4, u7), SoundBuzzer<'a>, 16>,
}

impl<'a> SongPlayer<'a> {
    fn new(buzzers: Deque<SoundBuzzer<'a>, 16>) -> Self {
        SongPlayer {
            instrument_sounds: [SoundProfile {
                wait_time: 3800,
                duration: None,
                wait_change_per_key: 50,
            }; 16],
            free_buzzers: buzzers,
            taken_buzzers: LinearMap::new(),
        }
    }

    fn reset(&mut self) {
        let mut keys = Deque::<(u4, u7), 16>::new();

        for key in self.taken_buzzers.keys() {
            if keys.push_back(*key).is_err() {
                break;
            }
        }
        while let Some(key) = keys.pop_front() {
            if let Some(mut taken_buzzer) = self.taken_buzzers.remove(&key) {
                taken_buzzer.reset();
                let _ = self.free_buzzers.push_back(taken_buzzer);
            }
        }
    }

    fn play_buzzers(&mut self) {
        for buzzer in self.taken_buzzers.values_mut() {
            buzzer.update();
        }
    }

    fn free_buzzers(&mut self) {
        let mut freed_keys = Deque::<(u4, u7), 16>::new();

        for key in self
            .taken_buzzers
            .iter()
            .filter(|(_, buzzer)| buzzer.max_period < 0)
            .map(|(key, _)| key)
        {
            if freed_keys.push_back(*key).is_err() {
                break;
            }
        }
        while let Some(key) = freed_keys.pop_front() {
            if let Some(mut taken_buzzer) = self.taken_buzzers.remove(&key) {
                taken_buzzer.reset();
                let _ = self.free_buzzers.push_back(taken_buzzer);
            }
        }
    }

    const fn delta_to_micros(delta_ticks: u16, meta_data: &SongMetaData) -> u64 {
        (delta_ticks as u64 * meta_data.tempo as u64) / meta_data.ticks_per_quarter as u64
    }

    fn play_song(&mut self, midi_track: &[u8]) {
        // ------------------- parse the track -------------------

        let (header, track_iter) = parse(midi_track).expect("valid midi track");
        let mut metadata = SongMetaData::new(header);

        let mut next_events: [Option<(u16, TrackEventKind<'_>)>; 16] = [None; 16];

        // todo: take while delta = 0 from first track, see if there are meta info there, maybe
        let mut tracks: Vec<EventIter<'_>, 16> = track_iter.flatten().collect();

        for (i, t) in tracks.iter_mut().enumerate() {
            if let Some(Ok(first_event)) = t.next() {
                next_events[i] = Some((first_event.delta.as_int() as u16, first_event.kind));
            }
        }

        // ------------------- play all the track events in order -------------------
        while next_events.iter().any(|x| x.is_some()) {
            // pick the next event with the lowest delta
            let next_track_idx = Self::find_min_index(&next_events);
            let (delay, event_kind) = next_events[next_track_idx].expect("cannot fail");

            // ------------------- apply the delay to each of the items -------------------
            if delay != 0 {
                next_events
                    .iter_mut()
                    .filter_map(|event| event.as_mut())
                    .for_each(|inner_event| inner_event.0 -= delay);
            }

            // ------------------- replace the picked next event -------------------
            match tracks[next_track_idx].next() {
                Some(next_event) => {
                    let replacing_event = next_event.expect("invalid track event");
                    next_events[next_track_idx] =
                        Some((replacing_event.delta.as_int() as u16, replacing_event.kind));
                }
                None => next_events[next_track_idx] = None,
            }
            // println!(
            //     "delay: {:?}, tracknum: {:?}, event: {:?}",
            //     delay, next_track_idx, event_kind
            // );

            // ------------------- Wait until the delay is gone and play buzzers -------------------
            let arbitrary_len = 15.0;

            let mut delta_time =
                (Self::delta_to_micros(delay, &metadata) as f32 * arbitrary_len) as i64;
            println!("{}", delta_time);
            if self.taken_buzzers.is_empty() {
                while delta_time > 0 {
                    delta_time -= 25;
                    delay_cycles(290);
                }
            } else {
                while delta_time > 0 {
                    self.play_buzzers();
                    delay_cycles(870);
                    delta_time -= 100;
                }
            }

            // ------------------- handle the current event -------------------

            self.match_music_events(&mut metadata, event_kind);
        }
        self.reset();
    }

    fn match_music_events(&mut self, metadata: &mut SongMetaData, event_kind: TrackEventKind) {
        match event_kind {
            TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel } => {
                    println!("taken buzzers len: {}", self.taken_buzzers.len());
                    if let Some(mut free_buzzer) = self.taken_buzzers.remove(&(channel, key)) {
                        println!("buzzer removed");
                        free_buzzer.reset();
                        let _ = self.free_buzzers.push_back(free_buzzer);
                    }
                }
                MidiMessage::NoteOn { key, vel } => {
                    if let Some(mut free_buzzer) = self.free_buzzers.pop_front() {
                        //println!("temp change, instrument is whatever, change this back");
                        //let note_to_play: &SoundProfile = &INSTRUMENTS[8];
                        let note_to_play: &SoundProfile =
                            &self.instrument_sounds[channel.as_int() as usize];
                        free_buzzer.play_note(note_to_play, key);
                        let _ = self.taken_buzzers.insert((channel, key), free_buzzer);
                    } else {
                        println!("no free buzzers")
                    }
                }
                MidiMessage::ProgramChange { program } => {
                    // gets the instrument index for the channel
                    self.instrument_sounds[channel.as_int() as usize] =
                        INSTRUMENTS[program.as_int() as usize]
                }
                MidiMessage::Aftertouch { key, vel } => {
                    println!("not implemented: midi aftertouch")
                }
                MidiMessage::Controller { controller, value } => {
                    println!("not implemented: midi controller")
                }

                MidiMessage::ChannelAftertouch { vel } => {
                    println!("not implemented: midi channel aftertouch")
                }
                MidiMessage::PitchBend { bend } => println!("not implemented: midi pitch bend"),
            },
            TrackEventKind::Meta(meta_message) => match meta_message {
                MetaMessage::Tempo(tempo) => metadata.tempo = tempo.as_int(),
                MetaMessage::TimeSignature(a, b, c, d) => metadata.time_signature = [a, b, c, d],
                MetaMessage::KeySignature(key, sharp) => metadata.key = (key, sharp),
                MetaMessage::EndOfTrack => println!("End of track"),

                MetaMessage::InstrumentName(bytes) => println!("not implemented: name"),
                MetaMessage::TrackName(bytes) => println!("not implemented: name"),
                MetaMessage::MidiChannel(u4) => println!("not implemented: num midi channels"),
                MetaMessage::MidiPort(u7) => println!("not implemented: num midi ports"),
                MetaMessage::TrackNumber(_) => println!("not implemented: track number"),

                _ => {}
            },
            TrackEventKind::SysEx(_) => {}
            TrackEventKind::Escape(_) => {}
        }
    }

    #[inline(always)]
    fn find_min_index(list: &[Option<(u16, TrackEventKind<'_>)>; 16]) -> usize {
        // find the index with the lowest u16
        let mut min_index = 0;
        let mut min_delta: u16 = u16::MAX;
        for (i, track_item) in list.iter().enumerate() {
            if let Some((delta, _)) = track_item {
                if delta == &0 {
                    return i;
                } else if *delta < min_delta {
                    min_index = i;
                    min_delta = *delta;
                }
            }
        }
        min_index
    }
}

// =============================================================================================
//                              ANALOG PIN WITH VALUES 0 - 255
// =============================================================================================

#[derive(Debug, Default)]
struct Analog8 {
    value: u8,
}

impl Analog8 {
    fn inc(&mut self) {
        self.value = self.value.saturating_add(10);
    }

    fn dec(&mut self) {
        self.value = self.value.saturating_sub(10);
    }
}

// =============================================================================================
//                           PIN OWNING BUZZERS FOR PLAYING NOTES
// =============================================================================================

struct SoundBuzzer<'a> {
    _buzzer_pin: Output<'a>,
    period_micros: u16,
    max_period: i32,
    current_micros: u16,
    pin_state: bool,
    pin_mask: u32,
}

impl<'a> SoundBuzzer<'a> {
    fn new(pin: AnyPin<'a>, pin_num: u32) -> Self {
        assert!((0..=31).contains(&pin_num)); // register only for pins 0 - 31
        Self {
            _buzzer_pin: Output::new(pin, Level::Low, OutputConfig::default()),
            period_micros: 4000,
            max_period: i32::MAX,
            current_micros: 0,
            pin_state: false,
            pin_mask: 1 << pin_num,
        }
    }

    fn reset(&mut self) {
        self.current_micros = 0;
        self.max_period = i32::MAX;

        // self.buzzer_pin.set_low();
        // self.pin_state = false;
    }

    fn play_note(&mut self, sound_profile: &SoundProfile, key: u7) {
        // key between 0 and 127, so 64 is the middle point
        // less than 64 = note goes down, so wait time goes up
        // more than 64 = note goes up, so wait time goes down

        self.period_micros = sound_profile.wait_time
            - (sound_profile.wait_change_per_key * (key.as_int() as u16 - 64));

        self.max_period = sound_profile.duration.unwrap_or(i32::MAX);
        println!("period micros: {}", self.period_micros);
    }

    #[inline(always)]
    fn update(&mut self) {
        // TODO: when changing the frequency to be from hz, remake this

        self.current_micros += 20;

        if self.max_period > 0 && self.current_micros > self.period_micros {
            const REGISTERS: [*mut u32; 2] = [GPIO_0_31_SET_REG, GPIO_0_31_CLEAR_REG];
            // we use unsafe instead of pin toggle because this is faster (measured)
            // and the speed is needed with possibly thousands of toggles per seconds
            // this is safe because the pin has been configured as an output and the buzzer owns the pin
            // so no one else has access to the pin and the pin state cannot change
            // we also guarantee that pin_num is always inside the valid registers (0..=31)

            // also REGISTERS is guaranteed to be 2 items, with index 0 and 1
            // so getting unchecked is fine when using a bool
            unsafe {
                // toggels pin on / off
                REGISTERS
                    .get_unchecked(self.pin_state as usize)
                    .write_volatile(self.pin_mask);
            }
            self.pin_state = !self.pin_state;
            self.current_micros = 0
        }
        self.max_period -= 5;
    }

    fn adjust_period(&mut self, delta: i16) {
        self.period_micros = self.period_micros.saturating_add_signed(delta);
        self.period_micros = self.period_micros.clamp(100, 20000);
        println!(
            "Period: {}us ({}Hz)",
            self.period_micros,
            1_000_000 / self.period_micros as u32
        );
    }
}

// =============================================================================================
//                                      KNOB ROTATION
// =============================================================================================

enum Rotation {
    Left,
    Right,
}

const fn get_knob_rotation(
    last_clk: bool,
    last_dt: bool,
    current_clk: bool,
    current_dt: bool,
) -> Option<Rotation> {
    match (last_clk, last_dt) {
        (true, true) => match (current_clk, current_dt) {
            (true, false) => Some(Rotation::Left),
            (false, true) => Some(Rotation::Right),
            (_, _) => None,
        },
        (false, false) => match (current_clk, current_dt) {
            (false, true) => Some(Rotation::Left),
            (true, false) => Some(Rotation::Right),
            (_, _) => None,
        },
        (true, false) => match (current_clk, current_dt) {
            (false, false) => Some(Rotation::Left),
            (true, true) => Some(Rotation::Right),
            (_, _) => None,
        },
        (false, true) => match (current_clk, current_dt) {
            (true, true) => Some(Rotation::Left),
            (false, false) => Some(Rotation::Right),
            (_, _) => None,
        },
    }
}

// =============================================================================================
//                                         MAIN
// =============================================================================================

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    // ---------- load track ----------

    //  let (header, track_iter) = parse(MIDI_DATA).unwrap();
    //  println!("track music data");
    //  for track_event in track_iter.clone().flatten() {
    //      for event in track_event.flatten() {
    //          println!("{:?}", event);
    //      }
    //  }

    // ---------- set up pins ----------

    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    // roatry encoder input pins

    let up_input_config = InputConfig::default();

    let clk = Input::new(peripherals.GPIO18, up_input_config.with_pull(Pull::Up));
    let dt = Input::new(peripherals.GPIO19, up_input_config.with_pull(Pull::Up));
    let sw = Input::new(peripherals.GPIO23, up_input_config.with_pull(Pull::Up));

    // ---------- set up analog DAC pins ----------

    let mut dac_25 = Dac::new(peripherals.DAC1, peripherals.GPIO25);

    // ---------- set baseline states ----------

    let buzzer_1 = SoundBuzzer::new(peripherals.GPIO5.degrade(), 5);
    let buzzer_2 = SoundBuzzer::new(peripherals.GPIO13.degrade(), 13);
    let buzzer_3 = SoundBuzzer::new(peripherals.GPIO14.degrade(), 14);
    let buzzer_4 = SoundBuzzer::new(peripherals.GPIO27.degrade(), 27);
    let buzzer_5 = SoundBuzzer::new(peripherals.GPIO16.degrade(), 16);
    let buzzer_6 = SoundBuzzer::new(peripherals.GPIO17.degrade(), 17);
    let buzzer_7 = SoundBuzzer::new(peripherals.GPIO26.degrade(), 26);
    let buzzer_8 = SoundBuzzer::new(peripherals.GPIO3.degrade(), 3);

    let mut analog_value_pin25 = Analog8::default();
    let mut buzzer_queue: Deque<SoundBuzzer, 16> = Deque::new();
    let _ = buzzer_queue.push_back(buzzer_1);
    //let _ = buzzer_queue.push_back(buzzer_2);
    //let _ = buzzer_queue.push_back(buzzer_3);
    //let _ = buzzer_queue.push_back(buzzer_4);
    //let _ = buzzer_queue.push_back(buzzer_5);
    //let _ = buzzer_queue.push_back(buzzer_6);
    //let _ = buzzer_queue.push_back(buzzer_7);
    //let _ = buzzer_queue.push_back(buzzer_8);

    let mut song_player = SongPlayer::new(buzzer_queue);
    // todo: add a self healing meachanism that tries to catch up / slow down to get the correct beat
    song_player.play_song(MIDI_DATA);

    dac_25.write(analog_value_pin25.value);

    // last states for rotary encode pins
    let mut last_clk_state = clk.is_high();
    let mut last_dt_state = dt.is_high();
    let mut last_sw_state = sw.is_low();

    let mut buzzer_0 = SoundBuzzer::new(peripherals.GPIO4.degrade(), 4);
    buzzer_0.max_period = 1_000_000_000;

    //buzzer_queue.push_back(buzzer_0);

    println!("song over");

    loop {
        // current states
        let current_clk_state = clk.is_high();
        let current_dt_state = dt.is_high();
        let current_sw_state = sw.is_low();

        // pin logic
        if sw.is_high() && current_sw_state != last_sw_state {
            led.toggle();
            buzzer_0.max_period = 1_000_000_000;

            println!("playing: {}", buzzer_0.max_period > 0)
        }

        if let Some(rotation) = get_knob_rotation(
            last_clk_state,
            last_dt_state,
            current_clk_state,
            current_dt_state,
        ) {
            match rotation {
                Rotation::Left => {
                    buzzer_0.adjust_period(20);
                    analog_value_pin25.dec();
                }
                Rotation::Right => {
                    buzzer_0.adjust_period(-20);
                    analog_value_pin25.inc();
                }
            }

            println!("analog led pin value: {}", analog_value_pin25.value);
            dac_25.write(analog_value_pin25.value);
        }

        buzzer_0.update();

        // reset current states
        last_dt_state = current_dt_state;
        last_sw_state = current_sw_state;
        last_clk_state = current_clk_state;

        delay_cycles(1700);
    }
}
