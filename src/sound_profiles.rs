// =============================================================================================
//                                SOUND PROFILE FOR INSTRUMENTS
// =============================================================================================

#[derive(Debug, Clone, Copy)]
pub struct SoundProfile {
    pub wait_time: u16,
    pub duration: Option<i32>,
    pub key_micro_chance: u8,
}

impl SoundProfile {
    const fn new(wait_time: u16, duration: Option<i32>, key_micro_chance: u8) -> Self {
        SoundProfile {
            wait_time,
            duration,
            key_micro_chance,
        }
    }
}

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

pub const INSTRUMENTS: [SoundProfile; 128] = [
    //  ======== Piano ========

    // Acoustic Grand
    SoundProfile::new(3800, None, 50),
    // Bright Acoustic
    SoundProfile::new(3800, None, 50),
    // Electric Grand
    SoundProfile::new(3800, None, 50),
    // Honky-Tonk
    SoundProfile::new(3800, None, 50),
    // Electric Piano 1
    SoundProfile::new(3800, None, 50),
    // Electric Piano 2
    SoundProfile::new(3800, None, 50),
    // Harpsichord
    SoundProfile::new(3800, None, 50),
    // Clavinet
    SoundProfile::new(3800, None, 50),
    //  ======== Chromatic Percussion ========

    // Celesta
    SoundProfile::new(3800, None, 50),
    // Glockenspiel
    SoundProfile::new(3800, None, 50),
    // Music Box
    SoundProfile::new(3800, None, 50),
    // Vibraphone
    SoundProfile::new(3800, None, 50),
    // Marimba
    SoundProfile::new(3800, None, 50),
    // Xylophone
    SoundProfile::new(3800, None, 50),
    // Tubular Bells
    SoundProfile::new(3800, None, 50),
    // Dulcimer
    SoundProfile::new(3800, None, 50),
    //  ======== Organ ========

    // Drawbar Organ
    SoundProfile::new(3800, None, 50),
    // Percussive Organ
    SoundProfile::new(3800, None, 50),
    // Rock Organ
    SoundProfile::new(3800, None, 50),
    // Church Organ
    SoundProfile::new(3800, None, 50),
    // Reed Organ
    SoundProfile::new(3800, None, 50),
    // Accordian
    SoundProfile::new(3800, None, 50),
    // Harmonica
    SoundProfile::new(3800, None, 50),
    // Tango Accordian
    SoundProfile::new(3800, None, 50),
    //  ======== Guitar ========

    // Nylon String Guitar
    SoundProfile::new(3800, None, 50),
    // Steel String Guitar
    SoundProfile::new(3800, None, 50),
    // Electric Jazz Guitar
    SoundProfile::new(3800, None, 50),
    // Electric Clean Guitar
    SoundProfile::new(3800, None, 50),
    // Electric Muted Guitar
    SoundProfile::new(3800, None, 50),
    // Overdriven Guitar
    SoundProfile::new(3800, None, 50),
    // Distortion Guitar
    SoundProfile::new(3800, None, 50),
    // Guitar Harmonics
    SoundProfile::new(3800, None, 50),
    //  ======== Bass ========

    // Acoustic Bass
    SoundProfile::new(3800, None, 50),
    // Electric Bass (finger)
    SoundProfile::new(3800, None, 50),
    // Electric Bass (pick)
    SoundProfile::new(3800, None, 50),
    // Fretless Bass
    SoundProfile::new(3800, None, 50),
    // Slap Bass 1
    SoundProfile::new(3800, None, 50),
    // Slap Bass 2
    SoundProfile::new(3800, None, 50),
    // Synth Bass 1
    SoundProfile::new(3800, None, 50),
    // Synth Bass 2
    SoundProfile::new(3800, None, 50),
    //  ======== Solo Strings ========

    // Violin
    SoundProfile::new(3800, None, 50),
    // Viola
    SoundProfile::new(3800, None, 50),
    // Cello
    SoundProfile::new(3800, None, 50),
    // Contrabass
    SoundProfile::new(3800, None, 50),
    // Tremolo Strings
    SoundProfile::new(3800, None, 50),
    // Pizzicato Strings
    SoundProfile::new(3800, None, 50),
    // Orchestral Strings
    SoundProfile::new(3800, None, 50),
    // Timpani
    SoundProfile::new(3800, None, 50),
    //  ======== Ensemble ========

    // String Ensemble 1
    SoundProfile::new(3800, None, 50),
    // String Ensemble 2
    SoundProfile::new(3800, None, 50),
    // SynthStrings 1
    SoundProfile::new(3800, None, 50),
    // SynthStrings 2
    SoundProfile::new(3800, None, 50),
    // Choir Aahs
    SoundProfile::new(3800, None, 50),
    // Voice Oohs
    SoundProfile::new(3800, None, 50),
    // Synth Voice
    SoundProfile::new(3800, None, 50),
    // Orchestra Hit
    SoundProfile::new(3800, None, 50),
    //  ======== Brass ========

    // Trumpet
    SoundProfile::new(3800, None, 50),
    // Trombone
    SoundProfile::new(3800, None, 50),
    // Tuba
    SoundProfile::new(3800, None, 50),
    // Muted Trumpet
    SoundProfile::new(3800, None, 50),
    // French Horn
    SoundProfile::new(3800, None, 50),
    // Brass Section
    SoundProfile::new(3800, None, 50),
    // SynthBrass 1
    SoundProfile::new(3800, None, 50),
    // SynthBrass 2
    SoundProfile::new(3800, None, 50),
    //  ======== Reed ========

    // Soprano Sax
    SoundProfile::new(3800, None, 50),
    // Alto Sax
    SoundProfile::new(3800, None, 50),
    // Tenor Sax
    SoundProfile::new(3800, None, 50),
    // Baritone Sax
    SoundProfile::new(3800, None, 50),
    // Oboe
    SoundProfile::new(3800, None, 50),
    // English Horn
    SoundProfile::new(3800, None, 50),
    // Bassoon
    SoundProfile::new(3800, None, 50),
    // Clarinet
    SoundProfile::new(3800, None, 50),
    //  ======== Pipe ========

    // Piccolo
    SoundProfile::new(3800, None, 50),
    // Flute
    SoundProfile::new(3800, None, 50),
    // Recorder
    SoundProfile::new(3800, None, 50),
    // Pan Flute
    SoundProfile::new(3800, None, 50),
    // Blown Bottle
    SoundProfile::new(3800, None, 50),
    // Shakuhachi
    SoundProfile::new(3800, None, 50),
    // Whistle
    SoundProfile::new(3800, None, 50),
    // Ocarina
    SoundProfile::new(3800, None, 50),
    //  ======== Synth Lead ========

    // Square Wave
    SoundProfile::new(3800, None, 50),
    // Saw Wave
    SoundProfile::new(3800, None, 50),
    // Syn. Calliope
    SoundProfile::new(3800, None, 50),
    // Chiffer Lead
    SoundProfile::new(3800, None, 50),
    // Charang
    SoundProfile::new(3800, None, 50),
    // Solo Vox
    SoundProfile::new(3800, None, 50),
    // 5th Saw Wave
    SoundProfile::new(3800, None, 50),
    // Bass & Lead
    SoundProfile::new(3800, None, 50),
    //  ======== Synth Pad ========

    // Fantasia
    SoundProfile::new(3800, None, 50),
    // Warm Pad
    SoundProfile::new(3800, None, 50),
    // Polysynth
    SoundProfile::new(3800, None, 50),
    // Space Voice
    SoundProfile::new(3800, None, 50),
    // Bowed Glass
    SoundProfile::new(3800, None, 50),
    // Metal Pad
    SoundProfile::new(3800, None, 50),
    // Halo Pad
    SoundProfile::new(3800, None, 50),
    // Sweep Pad
    SoundProfile::new(3800, None, 50),
    //  ======== Synth Effects ========

    // Ice Rain
    SoundProfile::new(3800, None, 50),
    // Soundtrack
    SoundProfile::new(3800, None, 50),
    // Crystal
    SoundProfile::new(3800, None, 50),
    // Atmosphere
    SoundProfile::new(3800, None, 50),
    // Brightness
    SoundProfile::new(3800, None, 50),
    // Goblin
    SoundProfile::new(3800, None, 50),
    // Echo Drops
    SoundProfile::new(3800, None, 50),
    // Star Theme
    SoundProfile::new(3800, None, 50),
    //  ======== Ethnic ========

    // Sitar
    SoundProfile::new(3800, None, 50),
    // Banjo
    SoundProfile::new(3800, None, 50),
    // Shamisen
    SoundProfile::new(3800, None, 50),
    // Koto
    SoundProfile::new(3800, None, 50),
    // Kalimba
    SoundProfile::new(3800, None, 50),
    // Bagpipe
    SoundProfile::new(3800, None, 50),
    // Fiddle
    SoundProfile::new(3800, None, 50),
    // Shanai
    SoundProfile::new(3800, None, 50),
    //  ======== Percussive ========

    // Tinkle Bell
    SoundProfile::new(3800, None, 50),
    // Agogo
    SoundProfile::new(3800, None, 50),
    // Steel Drums
    SoundProfile::new(3800, None, 50),
    // Woodblock
    SoundProfile::new(3800, None, 50),
    // Taiko Drum
    SoundProfile::new(3800, None, 50),
    // Melodic Tom
    SoundProfile::new(3800, None, 50),
    // Synth Drum
    SoundProfile::new(3800, None, 50),
    // Reverse Cymbal
    SoundProfile::new(3800, None, 50),
    //  ======== Sound Effects ========

    // Guitar Fret Noise
    SoundProfile::new(3800, None, 50),
    // Breath Noise
    SoundProfile::new(3800, None, 50),
    // Seashore
    SoundProfile::new(3800, None, 50),
    // Bird Tweet
    SoundProfile::new(3800, None, 50),
    // Telephone Ring
    SoundProfile::new(3800, None, 50),
    // Helicopter
    SoundProfile::new(3800, None, 50),
    // Applause
    SoundProfile::new(3800, None, 50),
    // Gunshot
    SoundProfile::new(3800, None, 50),
];
