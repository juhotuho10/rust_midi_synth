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
    SoundProfile::new(4700, None, 1.3),
    // 1. Bright Acoustic
    SoundProfile::new(4700, None, 1.3),
    // 2. Electric Grand
    SoundProfile::new(4100, None, 1.3),
    // 3. Honky-Tonk
    SoundProfile::new(3600, None, 1.3),
    // 4. Electric Piano 1
    SoundProfile::new(4700, None, 1.3),
    // 5. Electric Piano 2
    SoundProfile::new(3400, None, 1.3),
    // 6. Harpsichord
    SoundProfile::new(3000, None, 1.3),
    // 7. Clavinet
    SoundProfile::new(4700, None, 1.3),
    //  ======== Chromatic Percussion ========

    // 8. Celesta
    SoundProfile::new(2600, None, 1.3),
    // 9. Glockenspiel
    SoundProfile::new(2400, None, 1.3),
    // 10. Music Box
    SoundProfile::new(5000, None, 1.3),
    // 11. Vibraphone
    SoundProfile::new(3300, None, 1.3),
    // 12. Marimba
    SoundProfile::new(2400, None, 1.3),
    // 13. Xylophone
    SoundProfile::new(5700, None, 1.3),
    // 14. Tubular Bells
    SoundProfile::new(3200, None, 1.3),
    // 15. Dulcimer
    SoundProfile::new(3800, None, 1.3),
    //  ======== Organ ========

    // 16. Drawbar Organ
    SoundProfile::new(5100, None, 1.3),
    // 17. Percussive Organ
    SoundProfile::new(5600, None, 1.3),
    // 18. Rock Organ
    SoundProfile::new(5300, None, 1.3),
    // 19. Church Organ
    SoundProfile::new(4500, None, 1.3),
    // 20. Reed Organ
    SoundProfile::new(4000, None, 1.3),
    // 21. Accordian
    SoundProfile::new(4400, None, 1.3),
    // 22. Harmonica
    SoundProfile::new(3800, None, 1.3),
    // 23. Tango Accordian
    SoundProfile::new(3600, None, 1.3),
    //  ======== Guitar ========

    // 24. Nylon String Guitar
    SoundProfile::new(4400, None, 1.3),
    // 25. Steel String Guitar
    SoundProfile::new(3600, None, 1.3),
    // 26. Electric Jazz Guitar
    SoundProfile::new(4400, None, 1.3),
    // 27. Electric Clean Guitar
    SoundProfile::new(3500, None, 1.3),
    // 28. Electric Muted Guitar
    SoundProfile::new(4700, None, 1.3),
    // 29. Overdriven Guitar
    SoundProfile::new(4400, None, 1.3),
    // 30. Distortion Guitar
    SoundProfile::new(4300, None, 1.3),
    // 31. Guitar Harmonics
    SoundProfile::new(3100, None, 1.3),
    //  ======== Bass ========

    // 32. Acoustic Bass
    SoundProfile::new(5500, None, 1.3),
    // 33. Electric Bass (finger)
    SoundProfile::new(5800, None, 1.3),
    // 34. Electric Bass (pick)
    SoundProfile::new(5100, None, 1.3),
    // 35. Fretless Bass
    SoundProfile::new(4500, None, 1.3),
    // 36. Slap Bass 1
    SoundProfile::new(4200, None, 1.3),
    // 37. Slap Bass 2
    SoundProfile::new(3800, None, 1.3),
    // 38. Synth Bass 1
    SoundProfile::new(3400, None, 1.3),
    // 39. Synth Bass 2
    SoundProfile::new(4100, None, 1.3),
    //  ======== Solo Strings ========

    // 40. Violin
    SoundProfile::new(3600, None, 1.3),
    // 41. Viola
    SoundProfile::new(4100, None, 1.3),
    // 42. Cello
    SoundProfile::new(3800, None, 1.3),
    // 43. Contrabass
    SoundProfile::new(3600, None, 1.3),
    // 44. Tremolo Strings
    SoundProfile::new(4600, None, 1.3),
    // 45. Pizzicato Strings
    SoundProfile::new(5100, None, 1.3),
    // 46. Orchestral Strings
    SoundProfile::new(4800, None, 1.3),
    // 47. Timpani
    SoundProfile::new(4000, None, 1.3),
    //  ======== Ensemble ========

    // 48. String Ensemble 1
    SoundProfile::new(4000, None, 1.3),
    // 49. String Ensemble 2
    SoundProfile::new(4000, None, 1.3),
    // 50. SynthStrings 1
    SoundProfile::new(4000, None, 1.3),
    // 51. SynthStrings 2
    SoundProfile::new(4000, None, 1.3),
    // 52. Choir Aahs
    SoundProfile::new(4700, None, 1.3),
    // 53. Voice Oohs
    SoundProfile::new(4700, None, 1.3),
    // 54. Synth Voice
    SoundProfile::new(4500, None, 1.3),
    // 55. Orchestra Hit
    SoundProfile::new(3900, None, 1.3),
    //  ======== Brass ========

    // 56. Trumpet
    SoundProfile::new(4600, None, 1.3),
    // 57. Trombone
    SoundProfile::new(4400, None, 1.3),
    // 58. Tuba
    SoundProfile::new(5200, None, 1.3),
    // 59. Muted Trumpet
    SoundProfile::new(3400, None, 1.3),
    // 60. French Horn
    SoundProfile::new(3600, None, 1.3),
    // 61. Brass Section
    SoundProfile::new(4400, None, 1.3),
    // 62. SynthBrass 1
    SoundProfile::new(4000, None, 1.3),
    // 63. SynthBrass 2
    SoundProfile::new(4500, None, 1.3),
    //  ======== Reed ========

    // 64. Soprano Sax
    SoundProfile::new(4900, None, 1.3),
    // 65. Alto Sax
    SoundProfile::new(4600, None, 1.3),
    // 66. Tenor Sax
    SoundProfile::new(4800, None, 1.3),
    // 67. Baritone Sax
    SoundProfile::new(4300, None, 1.3),
    // 68. Oboe
    SoundProfile::new(5200, None, 1.3),
    // 69. English Horn
    SoundProfile::new(5400, None, 1.3),
    // 70. Bassoon
    SoundProfile::new(5200, None, 1.3),
    // 71. Clarinet
    SoundProfile::new(5000, None, 1.3),
    //  ======== Pipe ========

    // 72. Piccolo
    SoundProfile::new(5900, None, 1.3),
    // 73. Flute
    SoundProfile::new(5700, None, 1.3),
    // 74. Recorder
    SoundProfile::new(5500, None, 1.3),
    // 75. Pan Flute
    SoundProfile::new(6100, None, 1.3),
    // 76. Blown Bottle
    SoundProfile::new(6100, None, 1.3),
    // 77. Shakuhachi
    SoundProfile::new(5200, None, 1.3),
    // 78. Whistle
    SoundProfile::new(6100, None, 1.3),
    // 79. Ocarina
    SoundProfile::new(6100, None, 1.3),
    //  ======== Synth Lead ========

    // 80. Square Wave
    SoundProfile::new(4200, None, 1.3),
    // 81. Saw Wave
    SoundProfile::new(3800, None, 1.3),
    // 82. Syn. Calliope
    SoundProfile::new(5500, None, 1.3),
    // 83. Chiffer Lead
    SoundProfile::new(4200, None, 1.3),
    // 84. Charang
    SoundProfile::new(3400, None, 1.3),
    // 85. Solo Vox
    SoundProfile::new(5500, None, 1.3),
    // 86. 5th Saw Wave
    SoundProfile::new(5200, None, 1.3),
    // 87. Bass & Lead
    SoundProfile::new(3600, None, 1.3),
    //  ======== Synth Pad ========

    // 88. Fantasia
    SoundProfile::new(3800, None, 1.3),
    // 89. Warm Pad
    SoundProfile::new(5400, None, 1.3),
    // 90. Polysynth
    SoundProfile::new(3800, None, 1.3),
    // 91. Space Voice
    SoundProfile::new(5400, None, 1.3),
    // 92. Bowed Glass
    SoundProfile::new(5400, None, 1.3),
    // 93. Metal Pad
    SoundProfile::new(5000, None, 1.3),
    // 94. Halo Pad
    SoundProfile::new(5400, None, 1.3),
    // 95. Sweep Pad
    SoundProfile::new(5500, None, 1.3),
    //  ======== Synth Effects ========

    // 96. Ice Rain
    SoundProfile::new(5500, None, 1.3),
    // 97. Soundtrack
    SoundProfile::new(4400, None, 1.3),
    // 98. Crystal
    SoundProfile::new(4700, None, 1.3),
    // 99. Atmosphere
    SoundProfile::new(4600, None, 1.3),
    // 100. Brightness
    SoundProfile::new(5500, None, 1.3),
    // 101. Goblin
    SoundProfile::new(5300, None, 1.3),
    // 102. Echo Drops
    SoundProfile::new(4600, None, 1.3),
    // 103. Star Theme
    SoundProfile::new(3300, None, 1.3),
    //  ======== Ethnic ========

    // 104. Sitar
    SoundProfile::new(3400, None, 1.3),
    // 105. Banjo
    SoundProfile::new(4200, None, 1.3),
    // 106. Shamisen
    SoundProfile::new(5500, None, 1.3),
    // 107. Koto
    SoundProfile::new(3400, None, 1.3),
    // 108. Kalimba
    SoundProfile::new(5200, None, 1.3),
    // 109. Bagpipe
    SoundProfile::new(4000, None, 1.3),
    // 110. Fiddle
    SoundProfile::new(4400, None, 1.3),
    // 111. Shanai
    SoundProfile::new(4000, None, 1.3),
    //  ======== Percussive ========

    // 112. Tinkle Bell
    SoundProfile::new(4200, None, 1.3),
    // 113. Agogo
    SoundProfile::new(3500, None, 1.3),
    // 114. Steel Drums
    SoundProfile::new(3800, None, 1.3),
    // 115. Woodblock
    SoundProfile::new(6700, None, 1.3),
    // 116. Taiko Drum
    SoundProfile::new(6000, None, 1.3),
    // 117. Melodic Tom
    SoundProfile::new(7000, None, 1.3),
    // 118. Synth Drum
    SoundProfile::new(7000, None, 1.3),
    // 119. Reverse Cymbal
    SoundProfile::new(8000, None, 1.3),
    //  ======== Sound Effects ========

    // 120. Guitar Fret Noise
    SoundProfile::new(4500, None, 1.3),
    // 121. Breath Noise
    SoundProfile::new(4100, None, 1.3),
    // 122. Seashore
    SoundProfile::new(3800, None, 1.3),
    // 123. Bird Tweet
    SoundProfile::new(3200, None, 1.3),
    // 124. Telephone Ring
    SoundProfile::new(3400, None, 1.3),
    // 125. Helicopter
    SoundProfile::new(3800, None, 1.3),
    // 126. Applause
    SoundProfile::new(7000, None, 1.3),
    // 127. Gunshot
    SoundProfile::new(6500, None, 1.3),
];
