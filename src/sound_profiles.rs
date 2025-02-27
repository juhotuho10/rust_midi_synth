// =============================================================================================
//                                SOUND PROFILE FOR INSTRUMENTS
// =============================================================================================

#[derive(Debug, Clone, Copy)]
pub struct SoundProfile {
    pub wait_time: u16,
    pub duration: Option<i32>,
    pub key_micro_change: u16,
}

impl SoundProfile {
    const fn new(wait_time: u16, duration: Option<i32>, change_percentage: f32) -> Self {
        let change = (wait_time as f32 * change_percentage) as u32;

        SoundProfile {
            wait_time,
            duration,
            key_micro_change: change as u16,
        }
    }
}

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

pub const INSTRUMENTS: [SoundProfile; 128] = [
    //  ======== Piano ========

    // Acoustic Grand
    SoundProfile::new(3800, None, 0.013),
    // Bright Acoustic
    SoundProfile::new(3800, None, 0.013),
    // Electric Grand
    SoundProfile::new(3800, None, 0.013),
    // Honky-Tonk
    SoundProfile::new(3800, None, 0.013),
    // Electric Piano 1
    SoundProfile::new(3800, None, 0.013),
    // Electric Piano 2
    SoundProfile::new(3800, None, 0.013),
    // Harpsichord
    SoundProfile::new(3800, None, 0.013),
    // Clavinet
    SoundProfile::new(3800, None, 0.013),
    //  ======== Chromatic Percussion ========

    // Celesta
    SoundProfile::new(3800, None, 0.013),
    // Glockenspiel
    SoundProfile::new(3800, None, 0.013),
    // Music Box
    SoundProfile::new(3800, None, 0.013),
    // Vibraphone
    SoundProfile::new(3800, None, 0.013),
    // Marimba
    SoundProfile::new(3800, None, 0.013),
    // Xylophone
    SoundProfile::new(3800, None, 0.013),
    // Tubular Bells
    SoundProfile::new(3800, None, 0.013),
    // Dulcimer
    SoundProfile::new(3800, None, 0.013),
    //  ======== Organ ========

    // Drawbar Organ
    SoundProfile::new(3800, None, 0.013),
    // Percussive Organ
    SoundProfile::new(3800, None, 0.013),
    // Rock Organ
    SoundProfile::new(3800, None, 0.013),
    // Church Organ
    SoundProfile::new(3800, None, 0.013),
    // Reed Organ
    SoundProfile::new(3800, None, 0.013),
    // Accordian
    SoundProfile::new(3800, None, 0.013),
    // Harmonica
    SoundProfile::new(3800, None, 0.013),
    // Tango Accordian
    SoundProfile::new(3800, None, 0.013),
    //  ======== Guitar ========

    // Nylon String Guitar
    SoundProfile::new(3800, None, 0.013),
    // Steel String Guitar
    SoundProfile::new(3800, None, 0.013),
    // Electric Jazz Guitar
    SoundProfile::new(3800, None, 0.013),
    // Electric Clean Guitar
    SoundProfile::new(3800, None, 0.013),
    // Electric Muted Guitar
    SoundProfile::new(3800, None, 0.013),
    // Overdriven Guitar
    SoundProfile::new(3800, None, 0.013),
    // Distortion Guitar
    SoundProfile::new(3800, None, 0.013),
    // Guitar Harmonics
    SoundProfile::new(3800, None, 0.013),
    //  ======== Bass ========

    // Acoustic Bass
    SoundProfile::new(3800, None, 0.013),
    // Electric Bass (finger)
    SoundProfile::new(3800, None, 0.013),
    // Electric Bass (pick)
    SoundProfile::new(3800, None, 0.013),
    // Fretless Bass
    SoundProfile::new(3800, None, 0.013),
    // Slap Bass 1
    SoundProfile::new(3800, None, 0.013),
    // Slap Bass 2
    SoundProfile::new(3800, None, 0.013),
    // Synth Bass 1
    SoundProfile::new(3800, None, 0.013),
    // Synth Bass 2
    SoundProfile::new(3800, None, 0.013),
    //  ======== Solo Strings ========

    // Violin
    SoundProfile::new(3800, None, 0.013),
    // Viola
    SoundProfile::new(3800, None, 0.013),
    // Cello
    SoundProfile::new(3800, None, 0.013),
    // Contrabass
    SoundProfile::new(3800, None, 0.013),
    // Tremolo Strings
    SoundProfile::new(3800, None, 0.013),
    // Pizzicato Strings
    SoundProfile::new(3800, None, 0.013),
    // Orchestral Strings
    SoundProfile::new(3800, None, 0.013),
    // Timpani
    SoundProfile::new(3800, None, 0.013),
    //  ======== Ensemble ========

    // String Ensemble 1
    SoundProfile::new(3800, None, 0.013),
    // String Ensemble 2
    SoundProfile::new(3800, None, 0.013),
    // SynthStrings 1
    SoundProfile::new(3800, None, 0.013),
    // SynthStrings 2
    SoundProfile::new(3800, None, 0.013),
    // Choir Aahs
    SoundProfile::new(3800, None, 0.013),
    // Voice Oohs
    SoundProfile::new(3800, None, 0.013),
    // Synth Voice
    SoundProfile::new(3800, None, 0.013),
    // Orchestra Hit
    SoundProfile::new(3800, None, 0.013),
    //  ======== Brass ========

    // Trumpet
    SoundProfile::new(3800, None, 0.013),
    // Trombone
    SoundProfile::new(3800, None, 0.013),
    // Tuba
    SoundProfile::new(3800, None, 0.013),
    // Muted Trumpet
    SoundProfile::new(3800, None, 0.013),
    // French Horn
    SoundProfile::new(3800, None, 0.013),
    // Brass Section
    SoundProfile::new(3800, None, 0.013),
    // SynthBrass 1
    SoundProfile::new(3800, None, 0.013),
    // SynthBrass 2
    SoundProfile::new(3800, None, 0.013),
    //  ======== Reed ========

    // Soprano Sax
    SoundProfile::new(3800, None, 0.013),
    // Alto Sax
    SoundProfile::new(3800, None, 0.013),
    // Tenor Sax
    SoundProfile::new(3800, None, 0.013),
    // Baritone Sax
    SoundProfile::new(3800, None, 0.013),
    // Oboe
    SoundProfile::new(3800, None, 0.013),
    // English Horn
    SoundProfile::new(3800, None, 0.013),
    // Bassoon
    SoundProfile::new(3800, None, 0.013),
    // Clarinet
    SoundProfile::new(3800, None, 0.013),
    //  ======== Pipe ========

    // Piccolo
    SoundProfile::new(3800, None, 0.013),
    // Flute
    SoundProfile::new(3800, None, 0.013),
    // Recorder
    SoundProfile::new(3800, None, 0.013),
    // Pan Flute
    SoundProfile::new(3800, None, 0.013),
    // Blown Bottle
    SoundProfile::new(3800, None, 0.013),
    // Shakuhachi
    SoundProfile::new(3800, None, 0.013),
    // Whistle
    SoundProfile::new(3800, None, 0.013),
    // Ocarina
    SoundProfile::new(3800, None, 0.013),
    //  ======== Synth Lead ========

    // Square Wave
    SoundProfile::new(3800, None, 0.013),
    // Saw Wave
    SoundProfile::new(3800, None, 0.013),
    // Syn. Calliope
    SoundProfile::new(3800, None, 0.013),
    // Chiffer Lead
    SoundProfile::new(3800, None, 0.013),
    // Charang
    SoundProfile::new(3800, None, 0.013),
    // Solo Vox
    SoundProfile::new(3800, None, 0.013),
    // 5th Saw Wave
    SoundProfile::new(3800, None, 0.013),
    // Bass & Lead
    SoundProfile::new(3800, None, 0.013),
    //  ======== Synth Pad ========

    // Fantasia
    SoundProfile::new(3800, None, 0.013),
    // Warm Pad
    SoundProfile::new(3800, None, 0.013),
    // Polysynth
    SoundProfile::new(3800, None, 0.013),
    // Space Voice
    SoundProfile::new(3800, None, 0.013),
    // Bowed Glass
    SoundProfile::new(3800, None, 0.013),
    // Metal Pad
    SoundProfile::new(3800, None, 0.013),
    // Halo Pad
    SoundProfile::new(3800, None, 0.013),
    // Sweep Pad
    SoundProfile::new(3800, None, 0.013),
    //  ======== Synth Effects ========

    // Ice Rain
    SoundProfile::new(3800, None, 0.013),
    // Soundtrack
    SoundProfile::new(3800, None, 0.013),
    // Crystal
    SoundProfile::new(3800, None, 0.013),
    // Atmosphere
    SoundProfile::new(3800, None, 0.013),
    // Brightness
    SoundProfile::new(3800, None, 0.013),
    // Goblin
    SoundProfile::new(3800, None, 0.013),
    // Echo Drops
    SoundProfile::new(3800, None, 0.013),
    // Star Theme
    SoundProfile::new(3800, None, 0.013),
    //  ======== Ethnic ========

    // Sitar
    SoundProfile::new(3800, None, 0.013),
    // Banjo
    SoundProfile::new(3800, None, 0.013),
    // Shamisen
    SoundProfile::new(3800, None, 0.013),
    // Koto
    SoundProfile::new(3800, None, 0.013),
    // Kalimba
    SoundProfile::new(3800, None, 0.013),
    // Bagpipe
    SoundProfile::new(3800, None, 0.013),
    // Fiddle
    SoundProfile::new(3800, None, 0.013),
    // Shanai
    SoundProfile::new(3800, None, 0.013),
    //  ======== Percussive ========

    // Tinkle Bell
    SoundProfile::new(3800, None, 0.013),
    // Agogo
    SoundProfile::new(3800, None, 0.013),
    // Steel Drums
    SoundProfile::new(3800, None, 0.013),
    // Woodblock
    SoundProfile::new(3800, None, 0.013),
    // Taiko Drum
    SoundProfile::new(3800, None, 0.013),
    // Melodic Tom
    SoundProfile::new(3800, None, 0.013),
    // Synth Drum
    SoundProfile::new(3800, None, 0.013),
    // Reverse Cymbal
    SoundProfile::new(3800, None, 0.013),
    //  ======== Sound Effects ========

    // Guitar Fret Noise
    SoundProfile::new(3800, None, 0.013),
    // Breath Noise
    SoundProfile::new(3800, None, 0.013),
    // Seashore
    SoundProfile::new(3800, None, 0.013),
    // Bird Tweet
    SoundProfile::new(3800, None, 0.013),
    // Telephone Ring
    SoundProfile::new(3800, None, 0.013),
    // Helicopter
    SoundProfile::new(3800, None, 0.013),
    // Applause
    SoundProfile::new(3800, None, 0.013),
    // Gunshot
    SoundProfile::new(3800, None, 0.013),
];
