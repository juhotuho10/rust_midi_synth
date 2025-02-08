#![no_std]
#![no_main]

use esp32_hal::adc::{AdcConfig, Attenuation, ADC, ADC2};
use esp32_hal::dac::DAC1;
use esp32_hal::gpio::{AnyPin, Output, PushPull};
use esp32_hal::ledc::channel::config::Config;
use esp32_hal::ledc::{channel, timer, LowSpeed, LEDC};
use esp32_hal::xtensa_lx_rt::entry;
use esp32_hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, timer::TimerGroup, Delay, Rtc,
};
use esp32_hal::{prelude::*, reset};
use esp_println::println;

use heapless::Deque;
use midly::{
    parse, EventIter, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind,
};
use panic_halt as _;

const MIDI_DATA: &[u8] = &[
    0x4d, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x00, 0x60, 0x4d, 0x54,
    0x72, 0x6b, 0x00, 0x00, 0x00, 0x5c, 0x00, 0xff, 0x01, 0x24, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x73,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f,
    0x77, 0x77, 0x77, 0x2e, 0x62, 0x65, 0x65, 0x70, 0x62, 0x6f, 0x78, 0x2e, 0x63, 0x6f, 0x00, 0xff,
    0x51, 0x03, 0x06, 0x1a, 0x80, 0x00, 0xff, 0x58, 0x04, 0x08, 0x02, 0x18, 0x08, 0x00, 0xff, 0x59,
    0x02, 0x00, 0x00, 0x00, 0xff, 0x06, 0x0a, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x86, 0x00, 0xff, 0x06, 0x08, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x45, 0x6e, 0x64, 0x00, 0xff,
    0x2f, 0x00, 0x4d, 0x54, 0x72, 0x6b, 0x00, 0x00, 0x00, 0x62, 0x00, 0xff, 0x03, 0x0e, 0x70, 0x69,
    0x74, 0x63, 0x68, 0x31, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x00, 0xb0, 0x65, 0x00,
    0x00, 0xb0, 0x64, 0x00, 0x00, 0xb0, 0x06, 0x18, 0x00, 0xb0, 0x26, 0x00, 0x00, 0xb0, 0x65, 0x7f,
    0x00, 0xb0, 0x64, 0x7f, 0x00, 0xff, 0x04, 0x0c, 0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x31, 0x00, 0xc0, 0x34, 0x00, 0xb0, 0x07, 0x5c, 0x00, 0xb0, 0x0a, 0x40, 0x00,
    0x90, 0x45, 0x5a, 0x30, 0x80, 0x45, 0x5a, 0x60, 0x90, 0x45, 0x5a, 0x00, 0x90, 0x43, 0x5a, 0x30,
    0x80, 0x45, 0x5a, 0x00, 0x80, 0x43, 0x5a, 0x84, 0x40, 0xff, 0x2f, 0x00,
];

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct SongMetaData {
    timing: u16,             // ticks per quarter note
    tempo: u32,              // micro seconds per quarter note
    bpm: u16,                // ms / min = 60_000_000, so BPM = 60_000_000 / tempo
    time_signature: [u8; 4], // [beats per measure, denominator of the time signature as 1/2^n,midi clock per quarter note, Number of Notated 32nd Notes in a MIDI Quarter Note]
    key: (i8, bool),         // ((-n_of_flats, + n_of_sharps), major / minor)
}

impl SongMetaData {
    fn new(header: Header, meta_events: EventIter) -> Self {
        let timing = match header.timing {
            Timing::Metrical(metric_timing) => metric_timing.as_int(),
            Timing::Timecode(_, _) => unimplemented!(),
        };

        let mut metadata = SongMetaData {
            timing,
            tempo: 500_000,                // default tempo
            bpm: 120,                      // default BPM
            time_signature: [4, 4, 24, 8], // default: 4/4
            key: (0, false),               // default: C major
        };

        for event in meta_events.flatten() {
            if let TrackEventKind::Meta(meta_event) = event.kind {
                match meta_event {
                    MetaMessage::Tempo(tempo) => {
                        metadata.tempo = tempo.as_int();
                    }
                    MetaMessage::TimeSignature(num, den, clocks, notes_32nd) => {
                        metadata.time_signature = [num, den, clocks, notes_32nd];
                    }
                    MetaMessage::KeySignature(sharps_flats, minor) => {
                        metadata.key = (sharps_flats, minor);
                    }
                    _ => (),
                }
            }
        }

        metadata.refresh_bpm(metadata.tempo);

        metadata
    }

    fn refresh_bpm(&mut self, tempo: u32) {
        let micros_per_min = 60_000_000;
        self.bpm = (micros_per_min / tempo) as u16;
    }
}

struct SongPlayer {
    SoundData: InstrumentSounds,
    Free_Buzzers: Deque<SoundBuzzer, 16>,
    Taken_Buzzers: Deque<SoundBuzzer, 16>,
}

impl SongPlayer {
    fn new(Buzzers: Deque<SoundBuzzer, 16>) -> Self {
        SongPlayer {
            SoundData: InstrumentSounds::new(),
            Free_Buzzers: Buzzers,
            Taken_Buzzers: Deque::new(),
        }
    }

    fn reset(&mut self) {
        while let Some(mut buzzer) = self.Taken_Buzzers.pop_front() {
            let _ = buzzer.buzzer_pin.set_low();
            let _ = self.Free_Buzzers.push_back(buzzer);
        }
    }

    fn play_song(&mut self, mut metadata: SongMetaData, mut song_events: EventIter) {
        for event in song_events.flatten() {
            let TrackEvent { delta, kind } = event;
            match kind {
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOff { key, vel } => todo!(),
                    MidiMessage::NoteOn { key, vel } => todo!(),
                    MidiMessage::Aftertouch { key, vel } => println!("not implemented"),
                    MidiMessage::Controller { controller, value } => println!("not implemented"),
                    MidiMessage::ProgramChange { program } => println!("not implemented"),
                    MidiMessage::ChannelAftertouch { vel } => println!("not implemented"),
                    MidiMessage::PitchBend { bend } => println!("not implemented"),
                },
                TrackEventKind::Meta(meta_message) => match meta_message {
                    MetaMessage::EndOfTrack => return self.reset(),
                    MetaMessage::Tempo(u24) => todo!(),
                    MetaMessage::SmpteOffset(smpte_time) => todo!(),
                    MetaMessage::MidiChannel(u4) => todo!(),
                    MetaMessage::MidiPort(u7) => todo!(),
                    MetaMessage::TimeSignature(a, b, c, d) => {
                        metadata.time_signature = [a, b, c, d]
                    }
                    MetaMessage::KeySignature(key, sharp) => metadata.key = (key, sharp),
                    MetaMessage::TrackNumber(_) => println!("not implemented"),
                    MetaMessage::Text(items) => println!("not implemented"),
                    MetaMessage::TrackName(items) => println!("not implemented"),
                    MetaMessage::InstrumentName(items) => println!("not implemented"),

                    _ => {}
                },
                _ => return self.reset(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SoundProfile {
    frequency: u16,
}

#[derive(Debug)]
struct InstrumentSounds {
    profiles: [SoundProfile; 128],
}

impl InstrumentSounds {
    fn new() -> Self {
        InstrumentSounds {
            profiles: [SoundProfile { frequency: 1000 }; 128],
        }
    }
}

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

struct SoundBuzzer {
    buzzer_pin: AnyPin<Output<PushPull>>,
    period_micros: u16,
    duty_percent: u8,
    current_micros: u16,
    output_state: bool,
}

impl SoundBuzzer {
    fn new(pin: AnyPin<Output<PushPull>>, period_micros: u16, duty_percent: u8) -> Self {
        Self {
            buzzer_pin: pin,
            period_micros,
            duty_percent: duty_percent.clamp(0, 100),
            current_micros: 0,
            output_state: false,
        }
    }

    fn update(&mut self, micros_elapsed: u16) {
        self.current_micros = (self.current_micros + micros_elapsed) % self.period_micros;

        let duty_micros = (self.period_micros as u32 * self.duty_percent as u32 / 100) as u16;
        let new_state = self.current_micros < duty_micros;
        if new_state != self.output_state {
            self.buzzer_pin.toggle().unwrap();
            self.output_state = new_state;
        }
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

    fn adjust_duty(&mut self, delta: i8) {
        self.duty_percent = (self.duty_percent as i16 + delta as i16).clamp(0, 100) as u8;
        println!("Duty cycle: {}%", self.duty_percent);
    }
}

enum Rotation {
    Left,
    Right,
}

fn get_knob_rotation(
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

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    // ---------- load track ----------

    let (header, track_iter) = parse(MIDI_DATA).unwrap();
    let mut track_iter = track_iter.flatten();
    println!("{:?}", header);
    let meta_info = track_iter.next().unwrap();

    let track = track_iter.next().unwrap();

    let track_meta_data = SongMetaData::new(header, meta_info);

    println!("{:?}", track_meta_data);

    println!("track music data");
    for track_event in track {
        println!("{:?}", track_event);
    }

    // ---------- set up clock ----------
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.LPWR);
    wdt.disable();
    rtc.rwdt.disable();

    // rtc.get_time_us()

    let delay = Delay::new(&clocks);

    // ---------- set up pins ----------

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut pin0 = io.pins.gpio0.into_push_pull_output();
    let mut led = io.pins.gpio2.into_push_pull_output();
    let pin26 = io.pins.gpio26.into_push_pull_output();

    // roatry encoder input pins
    let clk = io.pins.gpio5.into_pull_up_input();
    let dt = io.pins.gpio13.into_pull_up_input();
    let sw = io.pins.gpio12.into_pull_up_input();

    // ---------- set up analog ADC pin ----------

    //let analog = peripherals.SENS.split();

    //DAC1::dac(peripherals.AES.start(), io.pins.gpio25.into_analog());

    //let mut adc1_config = AdcConfig::new();

    //let mut pin25 =
    //    adc1_config.enable_pin(io.pins.gpio25.into_analog(), Attenuation::Attenuation11dB);

    //let mut adc2 = ADC::<ADC2>::adc(analog.adc2, adc1_config).unwrap();

    // ---------- set up analog DAC pins ----------

    let dac_pin = io.pins.gpio25.into_analog();
    let mut dac_25 = DAC1::dac(peripherals.AES, dac_pin).unwrap();

    // ---------- set up PWM for driving buzzer ----------

    //let ledc = LEDC::new(peripherals.LEDC, &clocks);

    //let mut buzzer_timer = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    //buzzer_timer
    //    .configure(timer::config::Config {
    //        duty: timer::config::Duty::Duty8Bit,
    //        clock_source: timer::LSClockSource::APBClk,
    //        frequency: 1000u32.Hz(),
    //    })
    //    .unwrap();

    //let mut buzzer_channel = ledc.get_channel(channel::Number::Channel1, buzzer_pin);
    //buzzer_channel
    //    .configure(channel::config::Config {
    //        timer: &buzzer_timer,
    //        duty_pct: 0,
    //        pin_config: channel::config::PinConfig::PushPull,
    //    })
    //    .unwrap();

    // ---------- set baseline states ----------

    let mut analog_value_pin25 = Analog8::default();
    let mut buzzer_queue: Deque<SoundBuzzer, 16> = Deque::new();
    let mut song_player = SongPlayer::new(buzzer_queue);

    dac_25.write(analog_value_pin25.value);

    // last states for rotary encode pins
    let mut last_clk_state = clk.is_high().unwrap();
    let mut last_dt_state = dt.is_high().unwrap();
    let mut last_sw_state = sw.is_low().unwrap();

    let mut buzzer_0 = SoundBuzzer::new(pin26.degrade(), 1000, 50);
    //buzzer_queue.push_back(buzzer_0);

    // todo: add a self healing meachanism that tries to catch up / slow down to get the correct beat

    loop {
        // current states
        let current_clk_state = clk.is_high().unwrap();
        let current_dt_state = dt.is_high().unwrap();
        let current_sw_state = sw.is_low().unwrap();

        // pin logic
        if sw.is_low().unwrap() && current_sw_state != last_sw_state {
            led.toggle().unwrap();

            // buzzer_channel.set_duty(128).unwrap();
            // delay.delay_ms(200u32);
            // buzzer_channel.set_duty(0).unwrap();
        }

        if let Some(rotation) = get_knob_rotation(
            last_clk_state,
            last_dt_state,
            current_clk_state,
            current_dt_state,
        ) {
            match rotation {
                Rotation::Left => {
                    if led.is_set_high().unwrap() {
                        buzzer_0.adjust_duty(-1);
                    } else {
                        buzzer_0.adjust_period(20);
                    }
                    analog_value_pin25.dec();
                }
                Rotation::Right => {
                    if led.is_set_high().unwrap() {
                        buzzer_0.adjust_duty(1);
                    } else {
                        buzzer_0.adjust_period(-20);
                    }
                    analog_value_pin25.inc();
                }
            }

            println!("analog led pin value: {}", analog_value_pin25.value);
            dac_25.write(analog_value_pin25.value);
        }

        buzzer_0.update(1);

        // reset current states
        last_dt_state = current_dt_state;
        last_sw_state = current_sw_state;
        last_clk_state = current_clk_state;

        delay.delay_micros(1);
    }
}
