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

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

#[derive(Debug)]
pub struct InstrumentSounds {
    pub profiles: FnvIndexMap<&'static str, SoundProfile, INSTRUMENT_COUNT>,
}

impl InstrumentSounds {
    pub fn new() -> Self {
        let mut map: FnvIndexMap<&'static str, SoundProfile, INSTRUMENT_COUNT> = FnvIndexMap::new();

        for &(k, v) in &INSTRUMENTS {
            let _ = map.insert(k, v);
        }

        Self { profiles: map }
    }

    pub fn bytes_to_instrument(&self, bytes: &[u8]) -> SoundProfile {
        let byte_vec = Vec::<u8, 32>::from_slice(bytes).unwrap();
        let byte_string = String::from_utf8(byte_vec).unwrap();

        match self.profiles.get(byte_string.as_str()) {
            Some(instrument_sound) => *instrument_sound,
            None => {
                println!("ERROR: instrument not found, returning random instrument");
                self.profiles["Acoustic Grand"]
            }
        }
    }
}

// =============================================================================================
//                                      INSTRUMENT NAMES
// =============================================================================================

pub const INSTRUMENTS: [(&str, SoundProfile); INSTRUMENT_COUNT] = [
    // Piano
    (
        "Acoustic Grand",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bright Acoustic",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Grand",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Honky-Tonk",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Piano 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Piano 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Harpsichord",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Clavinet",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Chromatic Percussion
    (
        "Celesta",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Glockenspiel",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Music Box",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Vibraphone",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Marimba",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Xylophone",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Tubular Bells",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Dulcimer",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Organ
    (
        "Drawbar Organ",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Percussive Organ",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Rock Organ",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Church Organ",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Reed Organ",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Accordian",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Harmonica",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Tango Accordian",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Guitar
    (
        "Nylon String Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Steel String Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Jazz Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Clean Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Muted Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Overdriven Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Distortion Guitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Guitar Harmonics",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Bass
    (
        "Acoustic Bass",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Bass (finger)",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Electric Bass (pick)",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Fretless Bass",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Slap Bass 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Slap Bass 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Synth Bass 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Synth Bass 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Solo Strings
    (
        "Violin",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Viola",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Cello",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Contrabass",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Tremolo Strings",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Pizzicato Strings",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Orchestral Strings",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Timpani",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Ensemble
    (
        "String Ensemble 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "String Ensemble 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "SynthStrings 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "SynthStrings 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Choir Aahs",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Voice Oohs",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Synth Voice",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Orchestra Hit",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Brass
    (
        "Trumpet",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Trombone",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Tuba",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Muted Trumpet",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "French Horn",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Brass Section",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "SynthBrass 1",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "SynthBrass 2",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Reed
    (
        "Soprano Sax",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Alto Sax",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Tenor Sax",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Baritone Sax",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Oboe",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "English Horn",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bassoon",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Clarinet",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Pipe
    (
        "Piccolo",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Flute",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Recorder",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Pan Flute",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Blown Bottle",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Shakuhachi",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Whistle",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Ocarina",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Synth Lead
    (
        "Square Wave",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Saw Wave",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Syn. Calliope",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Chiffer Lead",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Charang",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Solo Vox",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "5th Saw Wave",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bass & Lead",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Synth Pad
    (
        "Fantasia",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Warm Pad",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Polysynth",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Space Voice",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bowed Glass",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Metal Pad",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Halo Pad",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Sweep Pad",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Synth Effects
    (
        "Ice Rain",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Soundtrack",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Crystal",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Atmosphere",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Brightness",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Goblin",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Echo Drops",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Star Theme",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Ethnic
    (
        "Sitar",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Banjo",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Shamisen",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Koto",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Kalimba",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bagpipe",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Fiddle",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Shanai",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Percussive
    (
        "Tinkle Bell",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Agogo",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Steel Drums",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Woodblock",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Taiko Drum",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Melodic Tom",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Synth Drum",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Reverse Cymbal",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    // Sound Effects
    (
        "Guitar Fret Noise",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Breath Noise",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Seashore",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Bird Tweet",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Telephone Ring",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Helicopter",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Applause",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
    (
        "Gunshot",
        SoundProfile {
            frequency: 3800,
            duration: None,
        },
    ),
];
