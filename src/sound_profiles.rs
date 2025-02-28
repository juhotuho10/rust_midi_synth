// =============================================================================================
//                                SOUND PROFILE FOR INSTRUMENTS
// =============================================================================================

#[derive(Debug, Clone, Copy)]
pub struct SoundProfile {
    pub wait_time: u16,
    pub duration: Option<i32>,
    pub wait_change_per_key: u16,
}

impl SoundProfile {
    const fn new(wait_time: u16, duration: Option<i32>, change_percentage: f32) -> Self {
        let change = (wait_time as f32 * change_percentage / 100.0) as u32;

        SoundProfile {
            wait_time,
            duration,
            wait_change_per_key: change as u16,
        }
    }
}

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

pub const INSTRUMENTS: [SoundProfile; 128] = [
    //  ======== Piano ========

    // 0. Acoustic Grand
    SoundProfile::new(4700, None, 1.6),
    // 1. Bright Acoustic
    SoundProfile::new(4700, None, 1.6),
    // 2. Electric Grand
    SoundProfile::new(4100, None, 1.4),
    // 3. Honky-Tonk
    SoundProfile::new(3600, None, 1.2),
    // 4. Electric Piano 1
    SoundProfile::new(4700, None, 1.5),
    // 5. Electric Piano 2
    SoundProfile::new(3400, None, 1.5),
    // 6. Harpsichord
    SoundProfile::new(3000, None, 2.0),
    // 7. Clavinet
    SoundProfile::new(4700, None, 2.0),
    //  ======== Chromatic Percussion ========

    // 8. Celesta
    SoundProfile::new(2600, None, 2.5),
    // 9. Glockenspiel
    SoundProfile::new(2400, None, 2.5),
    // 10. Music Box
    SoundProfile::new(5000, None, 2.5),
    // 11. Vibraphone
    SoundProfile::new(3300, None, 1.3),
    // 12. Marimba
    SoundProfile::new(2400, None, 1.5),
    // 13. Xylophone
    SoundProfile::new(5700, None, 0.8),
    // 14. Tubular Bells
    SoundProfile::new(3200, None, 2.6),
    // 15. Dulcimer
    SoundProfile::new(3800, None, 1.1),
    //  ======== Organ ========

    // 16. Drawbar Organ
    SoundProfile::new(5100, None, 2.0),
    // 17. Percussive Organ
    SoundProfile::new(5600, None, 0.7),
    // 18. Rock Organ
    SoundProfile::new(5300, None, 1.5),
    // 19. Church Organ
    SoundProfile::new(4500, None, 2.5),
    // 20. Reed Organ
    SoundProfile::new(4000, None, 1.5),
    // 21. Accordian
    SoundProfile::new(4400, None, 2.0),
    // 22. Harmonica
    SoundProfile::new(3800, None, 2.5),
    // 23. Tango Accordian
    SoundProfile::new(3600, None, 2.0),
    //  ======== Guitar ========

    // 24. Nylon String Guitar
    SoundProfile::new(4400, None, 1.5),
    // 25. Steel String Guitar
    SoundProfile::new(3600, None, 2.5),
    // 26. Electric Jazz Guitar
    SoundProfile::new(4400, None, 1.5),
    // 27. Electric Clean Guitar
    SoundProfile::new(3500, None, 1.0),
    // 28. Electric Muted Guitar
    SoundProfile::new(4700, None, 3.0),
    // 29. Overdriven Guitar
    SoundProfile::new(4400, None, 2.5),
    // 30. Distortion Guitar
    SoundProfile::new(4300, None, 2.5),
    // 31. Guitar Harmonics
    SoundProfile::new(3100, None, 3.5),
    //  ======== Bass ========

    // 32. Acoustic Bass
    SoundProfile::new(5500, None, 1.0),
    // 33. Electric Bass (finger)
    SoundProfile::new(5800, None, 1.0),
    // 34. Electric Bass (pick)
    SoundProfile::new(5100, None, 2.5),
    // 35. Fretless Bass
    SoundProfile::new(4500, None, 3.5),
    // 36. Slap Bass 1
    SoundProfile::new(4200, None, 1.0),
    // 37. Slap Bass 2
    SoundProfile::new(3800, None, 2.5),
    // 38. Synth Bass 1
    SoundProfile::new(3400, None, 2.5),
    // 39. Synth Bass 2
    SoundProfile::new(4100, None, 2.5),
    //  ======== Solo Strings ========

    // 40. Violin
    SoundProfile::new(3600, None, 3.5),
    // 41. Viola
    SoundProfile::new(4100, None, 2.0),
    // 42. Cello
    SoundProfile::new(3800, None, 2.5),
    // 43. Contrabass
    SoundProfile::new(3600, None, 3.5),
    // 44. Tremolo Strings
    SoundProfile::new(4600, None, 1.5),
    // 45. Pizzicato Strings
    SoundProfile::new(5100, None, 1.5),
    // 46. Orchestral Strings
    SoundProfile::new(4800, None, 2.5),
    // 47. Timpani
    SoundProfile::new(4000, None, 2.0),
    //  ======== Ensemble ========

    // 48. String Ensemble 1
    SoundProfile::new(4000, None, 2.5),
    // 49. String Ensemble 2
    SoundProfile::new(4000, None, 2.5),
    // 50. SynthStrings 1
    SoundProfile::new(4000, None, 3.5),
    // 51. SynthStrings 2
    SoundProfile::new(4000, None, 3.5),
    // 52. Choir Aahs
    SoundProfile::new(4700, None, 2.5),
    // 53. Voice Oohs
    SoundProfile::new(4700, None, 3.5),
    // 54. Synth Voice
    SoundProfile::new(4500, None, 2.0),
    // 55. Orchestra Hit
    SoundProfile::new(3900, None, 2.0),
    //  ======== Brass ========

    // 56. Trumpet
    SoundProfile::new(4600, None, 2.5),
    // 57. Trombone
    SoundProfile::new(4400, None, 3.5),
    // 58. Tuba
    SoundProfile::new(5200, None, 2.5),
    // 59. Muted Trumpet
    SoundProfile::new(3400, None, 2.5),
    // 60. French Horn
    SoundProfile::new(3600, None, 1.5),
    // 61. Brass Section
    SoundProfile::new(4400, None, 2.5),
    // 62. SynthBrass 1
    SoundProfile::new(4000, None, 2.5),
    // 63. SynthBrass 2
    SoundProfile::new(4500, None, 1.0),
    //  ======== Reed ========

    // 64. Soprano Sax
    SoundProfile::new(4900, None, 3.5),
    // 65. Alto Sax
    SoundProfile::new(4600, None, 2.5),
    // 66. Tenor Sax
    SoundProfile::new(4800, None, 2.5),
    // 67. Baritone Sax
    SoundProfile::new(4300, None, 2.5),
    // 68. Oboe
    SoundProfile::new(5200, None, 1.5),
    // 69. English Horn
    SoundProfile::new(5400, None, 2.5),
    // 70. Bassoon
    SoundProfile::new(5200, None, 1.0),
    // 71. Clarinet
    SoundProfile::new(5000, None, 2.5),
    //  ======== Pipe ========

    // 72. Piccolo
    SoundProfile::new(5900, None, 2.5),
    // 73. Flute
    SoundProfile::new(5700, None, 3.5),
    // 74. Recorder
    SoundProfile::new(5500, None, 3.5),
    // 75. Pan Flute
    SoundProfile::new(6100, None, 1.0),
    // 76. Blown Bottle
    SoundProfile::new(6100, None, 1.0),
    // 77. Shakuhachi
    SoundProfile::new(5200, None, 2.5),
    // 78. Whistle
    SoundProfile::new(6100, None, 3.5),
    // 79. Ocarina
    SoundProfile::new(6100, None, 1.5),
    //  ======== Synth Lead ========

    // 80. Square Wave
    SoundProfile::new(4200, None, 1.5),
    // 81. Saw Wave
    SoundProfile::new(3800, None, 1.0),
    // 82. Syn. Calliope
    SoundProfile::new(5500, None, 1.5),
    // 83. Chiffer Lead
    SoundProfile::new(4200, None, 2.5),
    // 84. Charang
    SoundProfile::new(3400, None, 1.5),
    // 85. Solo Vox
    SoundProfile::new(5500, None, 2.5),
    // 86. 5th Saw Wave
    SoundProfile::new(5200, None, 3.5),
    // 87. Bass & Lead
    SoundProfile::new(3600, None, 3.5),
    //  ======== Synth Pad ========

    // 88. Fantasia
    SoundProfile::new(3800, None, 1.0),
    // 89. Warm Pad
    SoundProfile::new(5400, None, 2.5),
    // 90. Polysynth
    SoundProfile::new(3800, None, 0.5),
    // 91. Space Voice
    SoundProfile::new(5400, None, 1.5),
    // 92. Bowed Glass
    SoundProfile::new(5400, None, 2.5),
    // 93. Metal Pad
    SoundProfile::new(5000, None, 2.5),
    // 94. Halo Pad
    SoundProfile::new(5400, None, 2.5),
    // 95. Sweep Pad
    SoundProfile::new(5500, None, 3.5),
    //  ======== Synth Effects ========

    // 96. Ice Rain
    SoundProfile::new(5500, None, 2.5),
    // 97. Soundtrack
    SoundProfile::new(4400, None, 2.5),
    // 98. Crystal
    SoundProfile::new(4700, None, 1.0),
    // 99. Atmosphere
    SoundProfile::new(4600, None, 2.5),
    // 100. Brightness
    SoundProfile::new(5500, None, 1.5),
    // 101. Goblin
    SoundProfile::new(5300, None, 2.5),
    // 102. Echo Drops
    SoundProfile::new(4600, None, 2.5),
    // 103. Star Theme
    SoundProfile::new(3300, None, 2.5),
    //  ======== Ethnic ========

    // 104. Sitar
    SoundProfile::new(3400, None, 2.5),
    // 105. Banjo
    SoundProfile::new(4200, None, 2.5),
    // 106. Shamisen
    SoundProfile::new(5500, None, 1.5),
    // 107. Koto
    SoundProfile::new(3400, None, 2.5),
    // 108. Kalimba
    SoundProfile::new(5200, None, 2.5),
    // 109. Bagpipe
    SoundProfile::new(4000, None, 2.5),
    // 110. Fiddle
    SoundProfile::new(4400, None, 1.5),
    // 111. Shanai
    SoundProfile::new(4000, None, 2.5),
    //  ======== Percussive ========

    // 112. Tinkle Bell
    SoundProfile::new(4200, None, 2.5),
    // 113. Agogo
    SoundProfile::new(3500, None, 2.5),
    // 114. Steel Drums
    SoundProfile::new(3800, None, 2.5),
    // 115. Woodblock
    SoundProfile::new(6700, None, 0.5),
    // 116. Taiko Drum
    SoundProfile::new(6000, None, 1.5),
    // 117. Melodic Tom
    SoundProfile::new(7000, None, 0.5),
    // 118. Synth Drum
    SoundProfile::new(7000, None, 1.5),
    // 119. Reverse Cymbal
    SoundProfile::new(8000, None, 0.3),
    //  ======== Sound Effects ========

    // 120. Guitar Fret Noise
    SoundProfile::new(4500, None, 1.5),
    // 121. Breath Noise
    SoundProfile::new(4100, None, 2.5),
    // 122. Seashore
    SoundProfile::new(3800, None, 1.5),
    // 123. Bird Tweet
    SoundProfile::new(3200, None, 0.5),
    // 124. Telephone Ring
    SoundProfile::new(3400, None, 1.5),
    // 125. Helicopter
    SoundProfile::new(7000, None, 0.2),
    // 126. Applause
    SoundProfile::new(7000, None, 0.2),
    // 127. Gunshot
    SoundProfile::new(6500, None, 1.5),
];
