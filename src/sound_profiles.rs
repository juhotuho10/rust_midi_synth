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
        let change = (wait_time as f32 * change_percentage / 100.0) as u32;

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
    SoundProfile::new(3800, None, 1.3),
    // Bright Acoustic
    SoundProfile::new(3800, None, 1.3),
    // Electric Grand
    SoundProfile::new(3800, None, 1.3),
    // Honky-Tonk
    SoundProfile::new(3800, None, 1.3),
    // Electric Piano 1
    SoundProfile::new(3800, None, 1.3),
    // Electric Piano 2
    SoundProfile::new(3800, None, 1.3),
    // Harpsichord
    SoundProfile::new(3800, None, 1.3),
    // Clavinet
    SoundProfile::new(3800, None, 1.3),
    //  ======== Chromatic Percussion ========

    // Celesta
    SoundProfile::new(3800, None, 1.3),
    // Glockenspiel
    SoundProfile::new(3800, None, 1.3),
    // Music Box
    SoundProfile::new(3800, None, 1.3),
    // Vibraphone
    SoundProfile::new(3800, None, 1.3),
    // Marimba
    SoundProfile::new(3800, None, 1.3),
    // Xylophone
    SoundProfile::new(3800, None, 1.3),
    // Tubular Bells
    SoundProfile::new(3800, None, 1.3),
    // Dulcimer
    SoundProfile::new(3800, None, 1.3),
    //  ======== Organ ========

    // Drawbar Organ
    SoundProfile::new(3800, None, 1.3),
    // Percussive Organ
    SoundProfile::new(3800, None, 1.3),
    // Rock Organ
    SoundProfile::new(3800, None, 1.3),
    // Church Organ
    SoundProfile::new(3800, None, 1.3),
    // Reed Organ
    SoundProfile::new(3800, None, 1.3),
    // Accordian
    SoundProfile::new(3800, None, 1.3),
    // Harmonica
    SoundProfile::new(3800, None, 1.3),
    // Tango Accordian
    SoundProfile::new(3800, None, 1.3),
    //  ======== Guitar ========

    // Nylon String Guitar
    SoundProfile::new(3800, None, 1.3),
    // Steel String Guitar
    SoundProfile::new(3800, None, 1.3),
    // Electric Jazz Guitar
    SoundProfile::new(3800, None, 1.3),
    // Electric Clean Guitar
    SoundProfile::new(3800, None, 1.3),
    // Electric Muted Guitar
    SoundProfile::new(3800, None, 1.3),
    // Overdriven Guitar
    SoundProfile::new(3800, None, 1.3),
    // Distortion Guitar
    SoundProfile::new(3800, None, 1.3),
    // Guitar Harmonics
    SoundProfile::new(3800, None, 1.3),
    //  ======== Bass ========

    // Acoustic Bass
    SoundProfile::new(3800, None, 1.3),
    // Electric Bass (finger)
    SoundProfile::new(3800, None, 1.3),
    // Electric Bass (pick)
    SoundProfile::new(3800, None, 1.3),
    // Fretless Bass
    SoundProfile::new(3800, None, 1.3),
    // Slap Bass 1
    SoundProfile::new(3800, None, 1.3),
    // Slap Bass 2
    SoundProfile::new(3800, None, 1.3),
    // Synth Bass 1
    SoundProfile::new(3800, None, 1.3),
    // Synth Bass 2
    SoundProfile::new(3800, None, 1.3),
    //  ======== Solo Strings ========

    // Violin
    SoundProfile::new(3800, None, 1.3),
    // Viola
    SoundProfile::new(3800, None, 1.3),
    // Cello
    SoundProfile::new(3800, None, 1.3),
    // Contrabass
    SoundProfile::new(3800, None, 1.3),
    // Tremolo Strings
    SoundProfile::new(3800, None, 1.3),
    // Pizzicato Strings
    SoundProfile::new(3800, None, 1.3),
    // Orchestral Strings
    SoundProfile::new(3800, None, 1.3),
    // Timpani
    SoundProfile::new(3800, None, 1.3),
    //  ======== Ensemble ========

    // String Ensemble 1
    SoundProfile::new(3800, None, 1.3),
    // String Ensemble 2
    SoundProfile::new(3800, None, 1.3),
    // SynthStrings 1
    SoundProfile::new(3800, None, 1.3),
    // SynthStrings 2
    SoundProfile::new(3800, None, 1.3),
    // Choir Aahs
    SoundProfile::new(3800, None, 1.3),
    // Voice Oohs
    SoundProfile::new(3800, None, 1.3),
    // Synth Voice
    SoundProfile::new(3800, None, 1.3),
    // Orchestra Hit
    SoundProfile::new(3800, None, 1.3),
    //  ======== Brass ========

    // Trumpet
    SoundProfile::new(3800, None, 1.3),
    // Trombone
    SoundProfile::new(3800, None, 1.3),
    // Tuba
    SoundProfile::new(3800, None, 1.3),
    // Muted Trumpet
    SoundProfile::new(3800, None, 1.3),
    // French Horn
    SoundProfile::new(3800, None, 1.3),
    // Brass Section
    SoundProfile::new(3800, None, 1.3),
    // SynthBrass 1
    SoundProfile::new(3800, None, 1.3),
    // SynthBrass 2
    SoundProfile::new(3800, None, 1.3),
    //  ======== Reed ========

    // Soprano Sax
    SoundProfile::new(3800, None, 1.3),
    // Alto Sax
    SoundProfile::new(3800, None, 1.3),
    // Tenor Sax
    SoundProfile::new(3800, None, 1.3),
    // Baritone Sax
    SoundProfile::new(3800, None, 1.3),
    // Oboe
    SoundProfile::new(3800, None, 1.3),
    // English Horn
    SoundProfile::new(3800, None, 1.3),
    // Bassoon
    SoundProfile::new(3800, None, 1.3),
    // Clarinet
    SoundProfile::new(3800, None, 1.3),
    //  ======== Pipe ========

    // Piccolo
    SoundProfile::new(3800, None, 1.3),
    // Flute
    SoundProfile::new(3800, None, 1.3),
    // Recorder
    SoundProfile::new(3800, None, 1.3),
    // Pan Flute
    SoundProfile::new(3800, None, 1.3),
    // Blown Bottle
    SoundProfile::new(3800, None, 1.3),
    // Shakuhachi
    SoundProfile::new(3800, None, 1.3),
    // Whistle
    SoundProfile::new(3800, None, 1.3),
    // Ocarina
    SoundProfile::new(3800, None, 1.3),
    //  ======== Synth Lead ========

    // Square Wave
    SoundProfile::new(3800, None, 1.3),
    // Saw Wave
    SoundProfile::new(3800, None, 1.3),
    // Syn. Calliope
    SoundProfile::new(3800, None, 1.3),
    // Chiffer Lead
    SoundProfile::new(3800, None, 1.3),
    // Charang
    SoundProfile::new(3800, None, 1.3),
    // Solo Vox
    SoundProfile::new(3800, None, 1.3),
    // 5th Saw Wave
    SoundProfile::new(3800, None, 1.3),
    // Bass & Lead
    SoundProfile::new(3800, None, 1.3),
    //  ======== Synth Pad ========

    // Fantasia
    SoundProfile::new(3800, None, 1.3),
    // Warm Pad
    SoundProfile::new(3800, None, 1.3),
    // Polysynth
    SoundProfile::new(3800, None, 1.3),
    // Space Voice
    SoundProfile::new(3800, None, 1.3),
    // Bowed Glass
    SoundProfile::new(3800, None, 1.3),
    // Metal Pad
    SoundProfile::new(3800, None, 1.3),
    // Halo Pad
    SoundProfile::new(3800, None, 1.3),
    // Sweep Pad
    SoundProfile::new(3800, None, 1.3),
    //  ======== Synth Effects ========

    // Ice Rain
    SoundProfile::new(3800, None, 1.3),
    // Soundtrack
    SoundProfile::new(3800, None, 1.3),
    // Crystal
    SoundProfile::new(3800, None, 1.3),
    // Atmosphere
    SoundProfile::new(3800, None, 1.3),
    // Brightness
    SoundProfile::new(3800, None, 1.3),
    // Goblin
    SoundProfile::new(3800, None, 1.3),
    // Echo Drops
    SoundProfile::new(3800, None, 1.3),
    // Star Theme
    SoundProfile::new(3800, None, 1.3),
    //  ======== Ethnic ========

    // Sitar
    SoundProfile::new(3800, None, 1.3),
    // Banjo
    SoundProfile::new(3800, None, 1.3),
    // Shamisen
    SoundProfile::new(3800, None, 1.3),
    // Koto
    SoundProfile::new(3800, None, 1.3),
    // Kalimba
    SoundProfile::new(3800, None, 1.3),
    // Bagpipe
    SoundProfile::new(3800, None, 1.3),
    // Fiddle
    SoundProfile::new(3800, None, 1.3),
    // Shanai
    SoundProfile::new(3800, None, 1.3),
    //  ======== Percussive ========

    // Tinkle Bell
    SoundProfile::new(3800, None, 1.3),
    // Agogo
    SoundProfile::new(3800, None, 1.3),
    // Steel Drums
    SoundProfile::new(3800, None, 1.3),
    // Woodblock
    SoundProfile::new(3800, None, 1.3),
    // Taiko Drum
    SoundProfile::new(3800, None, 1.3),
    // Melodic Tom
    SoundProfile::new(3800, None, 1.3),
    // Synth Drum
    SoundProfile::new(3800, None, 1.3),
    // Reverse Cymbal
    SoundProfile::new(3800, None, 1.3),
    //  ======== Sound Effects ========

    // Guitar Fret Noise
    SoundProfile::new(3800, None, 1.3),
    // Breath Noise
    SoundProfile::new(3800, None, 1.3),
    // Seashore
    SoundProfile::new(3800, None, 1.3),
    // Bird Tweet
    SoundProfile::new(3800, None, 1.3),
    // Telephone Ring
    SoundProfile::new(3800, None, 1.3),
    // Helicopter
    SoundProfile::new(3800, None, 1.3),
    // Applause
    SoundProfile::new(3800, None, 1.3),
    // Gunshot
    SoundProfile::new(3800, None, 1.3),
];
