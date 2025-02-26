use esp_println::println;
use heapless::{FnvIndexMap, String, Vec};

const INSTRUMENT_COUNT: usize = 128;

// =============================================================================================
//                                SOUND PROFILE FOR INSTRUMENTS
// =============================================================================================

#[derive(Debug, Clone, Copy)]
pub struct SoundProfile {
    pub frequency: u16,
    pub duration: Option<i32>,
}

impl SoundProfile {
    const fn new(frequency: u16, duration: Option<i32>) -> Self {
        SoundProfile {
            frequency,
            duration,
        }
    }
}

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

// #[derive(Debug)]
// pub struct InstrumentSounds {
//     pub profiles: [SoundProfile; INSTRUMENT_COUNT],
// }
//
// impl InstrumentSounds {
//     pub fn new() -> Self {
//         Self {
//             profiles: INSTRUMENTS,
//         }
//     }
// }

// =============================================================================================
//                                      INSTRUMENT NAMES
// =============================================================================================

pub const INSTRUMENTS: [SoundProfile; INSTRUMENT_COUNT] = [
    //  ======== Piano ========

    // Acoustic Grand
    SoundProfile::new(3800, None),
    // Bright Acoustic
    SoundProfile::new(3800, None),
    // Electric Grand
    SoundProfile::new(3800, None),
    // Honky-Tonk
    SoundProfile::new(3800, None),
    // Electric Piano 1
    SoundProfile::new(3800, None),
    // Electric Piano 2
    SoundProfile::new(3800, None),
    // Harpsichord
    SoundProfile::new(3800, None),
    // Clavinet
    SoundProfile::new(3800, None),
    //  ======== Chromatic Percussion ========

    // Celesta
    SoundProfile::new(3800, None),
    // Glockenspiel
    SoundProfile::new(3800, None),
    // Music Box
    SoundProfile::new(3800, None),
    // Vibraphone
    SoundProfile::new(3800, None),
    // Marimba
    SoundProfile::new(3800, None),
    // Xylophone
    SoundProfile::new(3800, None),
    // Tubular Bells
    SoundProfile::new(3800, None),
    // Dulcimer
    SoundProfile::new(3800, None),
    //  ======== Organ ========

    // Drawbar Organ
    SoundProfile::new(3800, None),
    // Percussive Organ
    SoundProfile::new(3800, None),
    // Rock Organ
    SoundProfile::new(3800, None),
    // Church Organ
    SoundProfile::new(3800, None),
    // Reed Organ
    SoundProfile::new(3800, None),
    // Accordian
    SoundProfile::new(3800, None),
    // Harmonica
    SoundProfile::new(3800, None),
    // Tango Accordian
    SoundProfile::new(3800, None),
    //  ======== Guitar ========

    // Nylon String Guitar
    SoundProfile::new(3800, None),
    // Steel String Guitar
    SoundProfile::new(3800, None),
    // Electric Jazz Guitar
    SoundProfile::new(3800, None),
    // Electric Clean Guitar
    SoundProfile::new(3800, None),
    // Electric Muted Guitar
    SoundProfile::new(3800, None),
    // Overdriven Guitar
    SoundProfile::new(3800, None),
    // Distortion Guitar
    SoundProfile::new(3800, None),
    // Guitar Harmonics
    SoundProfile::new(3800, None),
    //  ======== Bass ========

    // Acoustic Bass
    SoundProfile::new(3800, None),
    // Electric Bass (finger)
    SoundProfile::new(3800, None),
    // Electric Bass (pick)
    SoundProfile::new(3800, None),
    // Fretless Bass
    SoundProfile::new(3800, None),
    // Slap Bass 1
    SoundProfile::new(3800, None),
    // Slap Bass 2
    SoundProfile::new(3800, None),
    // Synth Bass 1
    SoundProfile::new(3800, None),
    // Synth Bass 2
    SoundProfile::new(3800, None),
    //  ======== Solo Strings ========

    // Violin
    SoundProfile::new(3800, None),
    // Viola
    SoundProfile::new(3800, None),
    // Cello
    SoundProfile::new(3800, None),
    // Contrabass
    SoundProfile::new(3800, None),
    // Tremolo Strings
    SoundProfile::new(3800, None),
    // Pizzicato Strings
    SoundProfile::new(3800, None),
    // Orchestral Strings
    SoundProfile::new(3800, None),
    // Timpani
    SoundProfile::new(3800, None),
    //  ======== Ensemble ========

    // String Ensemble 1
    SoundProfile::new(3800, None),
    // String Ensemble 2
    SoundProfile::new(3800, None),
    // SynthStrings 1
    SoundProfile::new(3800, None),
    // SynthStrings 2
    SoundProfile::new(3800, None),
    // Choir Aahs
    SoundProfile::new(3800, None),
    // Voice Oohs
    SoundProfile::new(3800, None),
    // Synth Voice
    SoundProfile::new(3800, None),
    // Orchestra Hit
    SoundProfile::new(3800, None),
    //  ======== Brass ========

    // Trumpet
    SoundProfile::new(3800, None),
    // Trombone
    SoundProfile::new(3800, None),
    // Tuba
    SoundProfile::new(3800, None),
    // Muted Trumpet
    SoundProfile::new(3800, None),
    // French Horn
    SoundProfile::new(3800, None),
    // Brass Section
    SoundProfile::new(3800, None),
    // SynthBrass 1
    SoundProfile::new(3800, None),
    // SynthBrass 2
    SoundProfile::new(3800, None),
    //  ======== Reed ========

    // Soprano Sax
    SoundProfile::new(3800, None),
    // Alto Sax
    SoundProfile::new(3800, None),
    // Tenor Sax
    SoundProfile::new(3800, None),
    // Baritone Sax
    SoundProfile::new(3800, None),
    // Oboe
    SoundProfile::new(3800, None),
    // English Horn
    SoundProfile::new(3800, None),
    // Bassoon
    SoundProfile::new(3800, None),
    // Clarinet
    SoundProfile::new(3800, None),
    //  ======== Pipe ========

    // Piccolo
    SoundProfile::new(3800, None),
    // Flute
    SoundProfile::new(3800, None),
    // Recorder
    SoundProfile::new(3800, None),
    // Pan Flute
    SoundProfile::new(3800, None),
    // Blown Bottle
    SoundProfile::new(3800, None),
    // Shakuhachi
    SoundProfile::new(3800, None),
    // Whistle
    SoundProfile::new(3800, None),
    // Ocarina
    SoundProfile::new(3800, None),
    //  ======== Synth Lead ========

    // Square Wave
    SoundProfile::new(3800, None),
    // Saw Wave
    SoundProfile::new(3800, None),
    // Syn. Calliope
    SoundProfile::new(3800, None),
    // Chiffer Lead
    SoundProfile::new(3800, None),
    // Charang
    SoundProfile::new(3800, None),
    // Solo Vox
    SoundProfile::new(3800, None),
    // 5th Saw Wave
    SoundProfile::new(3800, None),
    // Bass & Lead
    SoundProfile::new(3800, None),
    //  ======== Synth Pad ========

    // Fantasia
    SoundProfile::new(3800, None),
    // Warm Pad
    SoundProfile::new(3800, None),
    // Polysynth
    SoundProfile::new(3800, None),
    // Space Voice
    SoundProfile::new(3800, None),
    // Bowed Glass
    SoundProfile::new(3800, None),
    // Metal Pad
    SoundProfile::new(3800, None),
    // Halo Pad
    SoundProfile::new(3800, None),
    // Sweep Pad
    SoundProfile::new(3800, None),
    //  ======== Synth Effects ========

    // Ice Rain
    SoundProfile::new(3800, None),
    // Soundtrack
    SoundProfile::new(3800, None),
    // Crystal
    SoundProfile::new(3800, None),
    // Atmosphere
    SoundProfile::new(3800, None),
    // Brightness
    SoundProfile::new(3800, None),
    // Goblin
    SoundProfile::new(3800, None),
    // Echo Drops
    SoundProfile::new(3800, None),
    // Star Theme
    SoundProfile::new(3800, None),
    //  ======== Ethnic ========

    // Sitar
    SoundProfile::new(3800, None),
    // Banjo
    SoundProfile::new(3800, None),
    // Shamisen
    SoundProfile::new(3800, None),
    // Koto
    SoundProfile::new(3800, None),
    // Kalimba
    SoundProfile::new(3800, None),
    // Bagpipe
    SoundProfile::new(3800, None),
    // Fiddle
    SoundProfile::new(3800, None),
    // Shanai
    SoundProfile::new(3800, None),
    //  ======== Percussive ========

    // Tinkle Bell
    SoundProfile::new(3800, None),
    // Agogo
    SoundProfile::new(3800, None),
    // Steel Drums
    SoundProfile::new(3800, None),
    // Woodblock
    SoundProfile::new(3800, None),
    // Taiko Drum
    SoundProfile::new(3800, None),
    // Melodic Tom
    SoundProfile::new(3800, None),
    // Synth Drum
    SoundProfile::new(3800, None),
    // Reverse Cymbal
    SoundProfile::new(3800, None),
    //  ======== Sound Effects ========

    // Guitar Fret Noise
    SoundProfile::new(3800, None),
    // Breath Noise
    SoundProfile::new(3800, None),
    // Seashore
    SoundProfile::new(3800, None),
    // Bird Tweet
    SoundProfile::new(3800, None),
    // Telephone Ring
    SoundProfile::new(3800, None),
    // Helicopter
    SoundProfile::new(3800, None),
    // Applause
    SoundProfile::new(3800, None),
    // Gunshot
    SoundProfile::new(3800, None),
];
