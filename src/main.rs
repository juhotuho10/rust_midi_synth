#![no_std]
#![no_main]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use esp_backtrace as _;

use esp_hal::{
    analog::dac::Dac,
    clock::CpuClock,
    delay::{self, Delay},
    gpio::{AnyPin, Event, Input, Io, Level, Output, Pin, Pull},
    ledc::{
        channel::{self, ChannelHW, ChannelIFace},
        timer::{self, HSClockSource, TimerIFace},
        HighSpeed, LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig},
    time::RateExtU32,
};

use esp_println::println;
use log::info;

use heapless::{Deque, LinearMap, String, Vec};

use midly::{
    num::{u28, u4, u7},
    parse, EventIter, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind,
    TrackIter,
};

// =============================================================================================
//                                      SONG HEX
// =============================================================================================

const MIDI_DATA: &[u8] = &[
    0x4d, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x00, 0x60, 0x4d, 0x54,
    0x72, 0x6b, 0x00, 0x00, 0x00, 0x5c, 0x00, 0xff, 0x01, 0x24, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x73,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f,
    0x77, 0x77, 0x77, 0x2e, 0x62, 0x65, 0x65, 0x70, 0x62, 0x6f, 0x78, 0x2e, 0x63, 0x6f, 0x00, 0xff,
    0x51, 0x03, 0x04, 0xb1, 0xec, 0x00, 0xff, 0x58, 0x04, 0x04, 0x02, 0x18, 0x08, 0x00, 0xff, 0x59,
    0x02, 0x03, 0x00, 0x00, 0xff, 0x06, 0x0a, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x53, 0x74, 0x61, 0x72,
    0x74, 0xfb, 0x00, 0xff, 0x06, 0x08, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x45, 0x6e, 0x64, 0x00, 0xff,
    0x2f, 0x00, 0x4d, 0x54, 0x72, 0x6b, 0x00, 0x00, 0x05, 0x95, 0x00, 0xff, 0x03, 0x0e, 0x70, 0x69,
    0x74, 0x63, 0x68, 0x31, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x00, 0xb0, 0x65, 0x00,
    0x00, 0xb0, 0x64, 0x00, 0x00, 0xb0, 0x06, 0x18, 0x00, 0xb0, 0x26, 0x00, 0x00, 0xb0, 0x65, 0x7f,
    0x00, 0xb0, 0x64, 0x7f, 0x00, 0xff, 0x04, 0x0c, 0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x31, 0x00, 0xc0, 0x44, 0x00, 0xb0, 0x07, 0x64, 0x00, 0xb0, 0x0a, 0x40, 0x00,
    0x90, 0x51, 0x5a, 0x18, 0x80, 0x51, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00,
    0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x81, 0x00, 0x80, 0x49, 0x5a,
    0x10, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a,
    0x00, 0x90, 0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x30, 0x80, 0x4e, 0x5a,
    0x00, 0x90, 0x50, 0x5a, 0x30, 0x80, 0x50, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x81, 0x28, 0x80, 0x4c,
    0x5a, 0x18, 0x90, 0x4e, 0x5a, 0x18, 0x80, 0x4e, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x30, 0x80, 0x4c,
    0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49,
    0x5a, 0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47,
    0x5a, 0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x4a, 0x5a, 0x81, 0x28, 0x80,
    0x4a, 0x5a, 0x18, 0x90, 0x51, 0x5a, 0x18, 0x80, 0x51, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x30, 0x80,
    0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x81, 0x04,
    0x80, 0x49, 0x5a, 0x3c, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30,
    0x80, 0x4a, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x30,
    0x80, 0x4e, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x81, 0x28, 0x80, 0x4a, 0x5a, 0x18, 0x90, 0x4e, 0x5a,
    0x30, 0x80, 0x4e, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a,
    0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a,
    0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a,
    0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x30, 0x80, 0x4e, 0x5a, 0x00, 0x90, 0x4c, 0x5a,
    0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a,
    0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x45, 0x5a,
    0x81, 0x28, 0x80, 0x45, 0x5a, 0x18, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47,
    0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x49,
    0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47,
    0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47,
    0x5a, 0x18, 0x80, 0x47, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x04, 0x80, 0x49, 0x5a, 0x3c, 0x90,
    0x45, 0x5a, 0x30, 0x80, 0x45, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90,
    0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90,
    0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47, 0x5a, 0x48, 0x90,
    0x49, 0x5a, 0x81, 0x28, 0x80, 0x49, 0x5a, 0x81, 0x58, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a,
    0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a,
    0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a,
    0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47, 0x5a,
    0x48, 0x90, 0x45, 0x5a, 0x18, 0x80, 0x45, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80, 0x49,
    0x5a, 0x18, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47,
    0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a,
    0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x45, 0x5a, 0x18, 0x80, 0x45,
    0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80, 0x49, 0x5a, 0x81, 0x58, 0x90, 0x4c, 0x5a, 0x30,
    0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x18,
    0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x30,
    0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x18,
    0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4e, 0x5a, 0x18, 0x80, 0x4e, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x81,
    0x28, 0x80, 0x4c, 0x5a, 0x18, 0x90, 0x50, 0x5a, 0x18, 0x80, 0x50, 0x5a, 0x48, 0x90, 0x50, 0x5a,
    0x30, 0x80, 0x50, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x30, 0x80, 0x4e, 0x5a, 0x00, 0x90, 0x4c, 0x5a,
    0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4a, 0x5a, 0x18, 0x80, 0x4a, 0x5a, 0x48, 0x90, 0x4c, 0x5a,
    0x81, 0x28, 0x80, 0x4c, 0x5a, 0x81, 0x58, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90,
    0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90,
    0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90,
    0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x18, 0x80, 0x4a, 0x5a, 0x48, 0x90,
    0x47, 0x5a, 0x18, 0x80, 0x47, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80, 0x49, 0x5a, 0x18,
    0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00,
    0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00,
    0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x45, 0x5a, 0x18, 0x80, 0x45, 0x5a, 0x48,
    0x90, 0x45, 0x5a, 0x18, 0x80, 0x45, 0x5a, 0x82, 0x68, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a,
    0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a,
    0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a,
    0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a,
    0x48, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x04, 0x80, 0x49,
    0x5a, 0x3c, 0x90, 0x45, 0x5a, 0x30, 0x80, 0x45, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47,
    0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a,
    0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47,
    0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80, 0x49, 0x5a, 0x81, 0x58, 0x90, 0x49, 0x5a, 0x30,
    0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18,
    0x80, 0x49, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x4a, 0x5a, 0x30,
    0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x18,
    0x80, 0x47, 0x5a, 0x48, 0x90, 0x45, 0x5a, 0x18, 0x80, 0x45, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81,
    0x28, 0x80, 0x49, 0x5a, 0x18, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a,
    0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a,
    0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x45, 0x5a,
    0x18, 0x80, 0x45, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80, 0x49, 0x5a, 0x81, 0x58, 0x90,
    0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90,
    0x4c, 0x5a, 0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4c, 0x5a, 0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90,
    0x4c, 0x5a, 0x30, 0x80, 0x4c, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80, 0x4a, 0x5a, 0x00, 0x90,
    0x4c, 0x5a, 0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4e, 0x5a, 0x18, 0x80, 0x4e, 0x5a, 0x48, 0x90,
    0x4c, 0x5a, 0x81, 0x28, 0x80, 0x4c, 0x5a, 0x18, 0x90, 0x50, 0x5a, 0x18, 0x80, 0x50, 0x5a, 0x48,
    0x90, 0x50, 0x5a, 0x30, 0x80, 0x50, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x30, 0x80, 0x4e, 0x5a, 0x00,
    0x90, 0x4c, 0x5a, 0x18, 0x80, 0x4c, 0x5a, 0x48, 0x90, 0x4a, 0x5a, 0x18, 0x80, 0x4a, 0x5a, 0x48,
    0x90, 0x4c, 0x5a, 0x81, 0x28, 0x80, 0x4c, 0x5a, 0x81, 0x58, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49,
    0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49,
    0x5a, 0x48, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x47, 0x5a, 0x30, 0x80, 0x47,
    0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x18, 0x80, 0x4a,
    0x5a, 0x48, 0x90, 0x47, 0x5a, 0x18, 0x80, 0x47, 0x5a, 0x48, 0x90, 0x49, 0x5a, 0x81, 0x28, 0x80,
    0x49, 0x5a, 0x18, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x30, 0x80,
    0x47, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x30, 0x80, 0x49, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x30, 0x80,
    0x4a, 0x5a, 0x00, 0x90, 0x49, 0x5a, 0x18, 0x80, 0x49, 0x5a, 0x48, 0x90, 0x45, 0x5a, 0x18, 0x80,
    0x45, 0x5a, 0x48, 0x90, 0x45, 0x5a, 0x18, 0x80, 0x45, 0x5a, 0x85, 0x68, 0xff, 0x2f, 0x00,
];

// =============================================================================================
//                                      INSTRUMENT NAMES
// =============================================================================================

const GM_INSTRUMENTS: [&str; 128] = [
    // Piano
    "Acoustic Grand",
    "Bright Acoustic",
    "Electric Grand",
    "Honky-Tonk",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clavinet",
    // Chromatic Percussion
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    // Organ
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accordian",
    "Harmonica",
    "Tango Accordian",
    // Guitar
    "Nylon String Guitar",
    "Steel String Guitar",
    "Electric Jazz Guitar",
    "Electric Clean Guitar",
    "Electric Muted Guitar",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    // Bass
    "Acoustic Bass",
    "Electric Bass (finger)",
    "Electric Bass (pick)",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    // Solo Strings
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Strings",
    "Timpani",
    // Ensemble
    "String Ensemble 1",
    "String Ensemble 2",
    "SynthStrings 1",
    "SynthStrings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    // Brass
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "SynthBrass 1",
    "SynthBrass 2",
    // Reed
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    // Pipe
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Shakuhachi",
    "Whistle",
    "Ocarina",
    // Synth Lead
    "Square Wave",
    "Saw Wave",
    "Syn. Calliope",
    "Chiffer Lead",
    "Charang",
    "Solo Vox",
    "5th Saw Wave",
    "Bass & Lead",
    // Synth Pad
    "Fantasia",
    "Warm Pad",
    "Polysynth",
    "Space Voice",
    "Bowed Glass",
    "Metal Pad",
    "Halo Pad",
    "Sweep Pad",
    // Synth Effects
    "Ice Rain",
    "Soundtrack",
    "Crystal",
    "Atmosphere",
    "Brightness",
    "Goblin",
    "Echo Drops",
    "Star Theme",
    // Ethnic
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bagpipe",
    "Fiddle",
    "Shanai",
    // Percussive
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    // Sound Effects
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot",
];

fn bytes_to_instrument_index(bytes: &[u8]) -> usize {
    let byte_vec = Vec::<u8, 32>::from_slice(bytes).unwrap();
    let byte_string = String::from_utf8(byte_vec).unwrap();
    // Find the index in the list

    match GM_INSTRUMENTS.iter().position(|item| item == &byte_string) {
        Some(index) => index,
        None => {
            println!("instrument not found: {}", byte_string);
            0
        }
    }
}

// =============================================================================================
//                         WRITE REGISTERS FOR PIN 0 - 31 FOR FAST TOGGLING
// =============================================================================================

const GPIO_0_31_SET_REG: *mut u32 = 0x3FF44008 as *mut u32; // set bit
const GPIO_0_31_CLEAR_REG: *mut u32 = 0x3FF4400C as *mut u32; // clear bit

// =============================================================================================
//                                      SONG METADATA
// =============================================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct SongMetaData {
    ticks_per_quarter: u16,  // ticks per quarter note
    tempo: u32,              // micro seconds per quarter note
    bpm: u16,                // ms / min = 60_000_000, so BPM = 60_000_000 / tempo
    time_signature: [u8; 4], // [beats per measure, denominator of the time signature as 1/2^n,midi clock per quarter note, Number of Notated 32nd Notes in a MIDI Quarter Note]
    key: (i8, bool),         // ((-n_of_flats, + n_of_sharps), major / minor)
}

impl SongMetaData {
    fn new_empty(header: Header) -> Self {
        let timing = match header.timing {
            Timing::Metrical(metric_timing) => metric_timing.as_int(),
            Timing::Timecode(_, _) => unimplemented!(),
        };

        SongMetaData {
            ticks_per_quarter: timing,
            tempo: 500_000,                // default tempo
            bpm: 120,                      // default BPM
            time_signature: [4, 4, 24, 8], // default: 4/4
            key: (0, false),               // default: C major
        }
    }

    fn new(header: Header, meta_events: EventIter) -> Self {
        let timing = match header.timing {
            Timing::Metrical(metric_timing) => metric_timing.as_int(),
            Timing::Timecode(_, _) => unimplemented!(),
        };

        let mut metadata = SongMetaData {
            ticks_per_quarter: timing,
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
        const MICROS_PER_MIN: u32 = 60_000_000;
        self.bpm = (MICROS_PER_MIN / tempo) as u16;
    }
}

// =============================================================================================
//                                      SONG PLAYER
// =============================================================================================

struct SongPlayer<'a> {
    sound_data: InstrumentSounds,
    free_buzzers: Deque<SoundBuzzer<'a>, 16>,
    taken_buzzers: LinearMap<(u4, u7), SoundBuzzer<'a>, 16>,
    delay: Delay,
}

impl<'a> SongPlayer<'a> {
    fn new(buzzers: Deque<SoundBuzzer<'a>, 16>) -> Self {
        SongPlayer {
            sound_data: InstrumentSounds::new(),
            free_buzzers: buzzers,
            taken_buzzers: LinearMap::new(),
            delay: Delay::new(),
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
            .filter(|(_, buzzer)| buzzer.finished_playing)
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

    fn delta_to_micros(delta_ticks: u16, meta_data: &SongMetaData) -> u64 {
        (delta_ticks as u64 * meta_data.tempo as u64) / meta_data.ticks_per_quarter as u64
    }

    fn play_song(&mut self, midi_track: &[u8]) {
        // ------------------- parse the track -------------------

        let (header, track_iter) = parse(midi_track).unwrap();
        let mut metadata = SongMetaData::new_empty(header);

        let mut next_events: [Option<(u16, TrackEventKind<'_>)>; 16] = [None; 16];

        // todo: take while delta = 0 from first track, see if there are meta info there
        let mut tracks: Vec<EventIter<'_>, 16> = track_iter.flatten().collect();

        for (i, t) in tracks.iter_mut().enumerate() {
            let first_event = t.next().unwrap().unwrap();
            next_events[i] = Some((first_event.delta.as_int() as u16, first_event.kind));
        }

        // ------------------- play all the track events in order -------------------
        loop {
            // pick the next event with the lowest delta
            let next_track_idx = Self::find_min_index(&next_events);
            let (delay, event_kind) = next_events[next_track_idx].unwrap();

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
                    delta_time -= 30;
                    self.delay.delay_nanos(100);
                }
            } else {
                while delta_time > 0 {
                    self.play_buzzers();
                    self.delay.delay_micros(5);
                    delta_time -= 100;
                }
            }

            // ------------------- handle the current event -------------------

            self.match_music_events(&mut metadata, event_kind);

            // if no events left, return
            if next_events.iter().all(|x| x.is_none()) {
                return;
            }
        }
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
                        free_buzzer.finished_playing = false;
                        let note_to_play = self.sound_data.profiles[0];
                        free_buzzer.playing_note = Some(Note {
                            duration: None, // none = indefinite
                            sound: note_to_play,
                            key: key.as_int(),
                        });

                        free_buzzer.set_frquency_from_note();

                        if self
                            .taken_buzzers
                            .insert((channel, key), free_buzzer)
                            .is_err()
                        {
                            println!("cannot insert buzzer");
                        };
                    } else {
                        println!("no free buzzers")
                    }
                }
                MidiMessage::Aftertouch { key, vel } => {
                    println!("not implemented: midi aftertouch")
                }
                MidiMessage::Controller { controller, value } => {
                    println!("not implemented: midi controller")
                }
                MidiMessage::ProgramChange { program } => {
                    println!("not implemented: midi program change")
                }
                MidiMessage::ChannelAftertouch { vel } => {
                    println!("not implemented: midi channel aftertouch")
                }
                MidiMessage::PitchBend { bend } => println!("not implemented: midi pitch bend"),
            },
            TrackEventKind::Meta(meta_message) => match meta_message {
                MetaMessage::EndOfTrack => self.reset(),
                MetaMessage::InstrumentName(items) => println!("not implemented: instrument name"), // todo: get the instruments that channels have
                MetaMessage::TrackName(items) => println!("not implemented: instrument name"),
                MetaMessage::Tempo(tempo) => metadata.tempo = tempo.as_int(),
                MetaMessage::SmpteOffset(smpte_time) => todo!(),
                MetaMessage::TimeSignature(a, b, c, d) => metadata.time_signature = [a, b, c, d],
                MetaMessage::KeySignature(key, sharp) => metadata.key = (key, sharp),

                MetaMessage::MidiChannel(u4) => println!("not implemented: num midi channels"),
                MetaMessage::MidiPort(u7) => println!("not implemented: num midi ports"),
                MetaMessage::TrackNumber(_) => println!("not implemented: track number"),
                MetaMessage::Text(items) => println!("not implemented: text"),

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
//                                SOUND PROFILE FOR INSTRUMENTS
// =============================================================================================

#[derive(Debug, Clone, Copy)]
struct SoundProfile {
    frequency: u16,
}

// =============================================================================================
//                        SOUND PROFILE COLLECTION FOR ALL INSTURMENTS
// =============================================================================================

#[derive(Debug)]
struct InstrumentSounds {
    profiles: [SoundProfile; 128],
}

impl InstrumentSounds {
    fn new() -> Self {
        InstrumentSounds {
            profiles: [SoundProfile { frequency: 3800 }; 128],
        }
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
//                       NOTES THAT BUZZERS CAN BE INSTRUCTED TO PLAY
// =============================================================================================

#[derive(Debug)]
struct Note {
    duration: Option<u16>,
    sound: SoundProfile,
    key: u8,
}

// =============================================================================================
//                           PIN OWNING BUZZERS FOR PLAYING NOTES
// =============================================================================================

struct SoundBuzzer<'a> {
    buzzer_pin: Output<'a>,
    period_micros: u16,
    half_period_micros: u16,
    current_micros: u16,
    playing_note: Option<Note>,
    pin_state: bool,
    finished_playing: bool,
    pin_mask: u32,
}

impl SoundBuzzer<'_> {
    fn new(pin: AnyPin, pin_num: u32, period_micros: u16) -> Self {
        assert!((0..=31).contains(&pin_num)); // register only for pins 0 - 31
        Self {
            buzzer_pin: Output::new(pin, Level::Low),
            period_micros,
            half_period_micros: (period_micros / 2),
            current_micros: 0,
            playing_note: None,
            pin_state: false,
            finished_playing: true,
            pin_mask: 1 << pin_num,
        }
    }

    fn reset(&mut self) {
        self.current_micros = 0;
        self.playing_note = None;
        self.finished_playing = true;
        // self.buzzer_pin.set_low();
        // self.pin_state = false;
    }

    fn set_frquency_from_note(&mut self) {
        if let Some(note) = &self.playing_note {
            self.period_micros = note.sound.frequency - ((6000 / 128) * (note.key as u16 - 64));
            println!("period micros: {}", self.period_micros);
        }
    }

    #[inline(always)]
    fn update(&mut self) {
        // TODO: when changing the frequency to be from hz, remake this

        self.current_micros += 20;
        if !self.finished_playing && self.current_micros > self.period_micros {
            const REGISTERS: [*mut u32; 2] = [GPIO_0_31_SET_REG, GPIO_0_31_CLEAR_REG];
            // we use unsafe instead of pin toggle because this is faster (measured)
            // and the speed is needed with possibly thousands of toggles per seconds
            // this is safe because the pin has been configured as an output and the buzzer owns the pin
            // so no one else has access to the pin and the pin state cannot change
            // we also guarantee that pin_num is always inside the valid registers (0..=31)
            unsafe {
                // toggels pin on / off
                REGISTERS[self.pin_state as usize].write_volatile(self.pin_mask);
            }
            self.pin_state = !self.pin_state;

            self.current_micros = 0
        }
    }

    fn adjust_period(&mut self, delta: i16) {
        self.period_micros = self.period_micros.saturating_add_signed(delta);
        self.period_micros = self.period_micros.clamp(100, 20000);
        self.half_period_micros = self.period_micros / 2;
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

// =============================================================================================
//                                         MAIN
// =============================================================================================

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();
    let delay = Delay::new();

    // ---------- load track ----------

    let (header, track_iter) = parse(MIDI_DATA).unwrap();

    //  println!("track music data");
    //  for track_event in track_iter.clone().flatten() {
    //      for event in track_event.flatten() {
    //          println!("{:?}", event);
    //      }
    //  }
    let mut track_iter = track_iter.flatten();
    println!("{:?}", header);
    let meta_info = track_iter.next().unwrap();

    let track = track_iter.next().unwrap();

    let mut track_meta_data = SongMetaData::new(header, meta_info);

    println!("{:?}", track_meta_data);

    // ---------- set up pins ----------

    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    // roatry encoder input pins
    let clk = Input::new(peripherals.GPIO19, Pull::Up);
    let dt = Input::new(peripherals.GPIO23, Pull::Up);
    let sw = Input::new(peripherals.GPIO5, Pull::Up);

    // ---------- set up analog DAC pins ----------

    let mut dac_25 = Dac::new(peripherals.DAC1, peripherals.GPIO25);

    // ---------- set up LEDC for driving buzzer ----------

    //    let mut ledc = Ledc::new(peripherals.LEDC);
    //    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    //    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    //    lstimer0
    //        .configure(timer::config::Config {
    //            duty: timer::config::Duty::Duty1Bit,
    //            clock_source: timer::LSClockSource::APBClk,
    //            frequency: 500.Hz(),
    //        })
    //        .unwrap();

    //    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    //    channel0
    //        .configure(channel::config::Config {
    //            timer: &lstimer0,
    //            duty_pct: 50,
    //            pin_config: channel::config::PinConfig::PushPull,
    //        })
    //        .unwrap();

    //    channel0.start_duty_fade(0, 100, 1).unwrap();

    //    loop {
    //        // Set up a breathing LED: fade from off to on over a second, then
    //        // from on back off over the next second.  Then loop2

    //        //channel0.start_duty_fade(0, 100, 1).unwrap();
    //        //while channel0.is_duty_fade_running() {}
    //        //channel0.start_duty_fade(100, 0, 1).unwrap();
    //        //while channel0.is_duty_fade_running() {}
    //    }

    // ---------- set up MCPWM for driving buzzer ----------

    //  let clock_cfg = PeripheralClockConfig::with_frequency(1.MHz()).unwrap();
    //  let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    //  // connect operator0 to timer0
    //  mcpwm.operator0.set_timer(&mcpwm.timer0);
    //  // connect operator0 to pin
    //  let mut pwm_pin = mcpwm
    //      .operator0
    //      .with_pin_a(pin0, PwmPinConfig::UP_DOWN_ACTIVE_HIGH);

    //  // start timer with timestamp values in the range of 0..=99 and a frequency
    //  // of 20 kHz
    //  let timer_clock_cfg = clock_cfg
    //      .timer_clock_with_frequency(50, PwmWorkingMode::Increase, 1.kHz())
    //      .unwrap();
    //  mcpwm.timer0.start(timer_clock_cfg);

    //  // pin will be high 50% of the time
    //  pwm_pin.set_timestamp(50);

    // ---------- set baseline states ----------

    let buzzer_1 = SoundBuzzer::new(peripherals.GPIO14.degrade(), 14, 2000);
    //let buzzer_2 = SoundBuzzer::new(peripherals.GPIO14.degrade(), 14, 2000);
    //let buzzer_3 = SoundBuzzer::new(peripherals.GPIO27.degrade(), 27, 2000);
    //let buzzer_4 = SoundBuzzer::new(peripherals.GPIO16.degrade(), 16, 2000);
    //let buzzer_5 = SoundBuzzer::new(peripherals.GPIO17.degrade(), 17, 2000);
    //let buzzer_6 = SoundBuzzer::new(peripherals.GPIO26.degrade(), 26, 2000);
    //let buzzer_7 = SoundBuzzer::new(peripherals.GPIO1.degrade(), 1, 2000);
    //let buzzer_8 = SoundBuzzer::new(peripherals.GPIO3.degrade(), 3, 2000);

    let mut analog_value_pin25 = Analog8::default();
    let mut buzzer_queue: Deque<SoundBuzzer, 16> = Deque::new();
    let _ = buzzer_queue.push_back(buzzer_1);
    // let _ = buzzer_queue.push_back(buzzer_2);
    // let _ = buzzer_queue.push_back(buzzer_3);
    // let _ = buzzer_queue.push_back(buzzer_4);
    // let _ = buzzer_queue.push_back(buzzer_5);
    // let _ = buzzer_queue.push_back(buzzer_6);
    // let _ = buzzer_queue.push_back(buzzer_7);
    // let _ = buzzer_queue.push_back(buzzer_8);

    let mut song_player = SongPlayer::new(buzzer_queue);
    // todo: add a self healing meachanism that tries to catch up / slow down to get the correct beat
    song_player.play_song(MIDI_DATA);

    dac_25.write(analog_value_pin25.value);

    // last states for rotary encode pins
    let mut last_clk_state = clk.is_high();
    let mut last_dt_state = dt.is_high();
    let mut last_sw_state = sw.is_low();

    let mut buzzer_0 = SoundBuzzer::new(peripherals.GPIO26.degrade(), 26, 2000);
    buzzer_0.finished_playing = false;
    //buzzer_queue.push_back(buzzer_0);

    println!("song over");

    loop {
        // current states
        let current_clk_state = clk.is_high();
        let current_dt_state = dt.is_high();
        let current_sw_state = sw.is_low();

        // pin logic
        if sw.is_low() && current_sw_state != last_sw_state {
            //led.toggle();
        }

        if let Some(rotation) = get_knob_rotation(
            last_clk_state,
            last_dt_state,
            current_clk_state,
            current_dt_state,
        ) {
            match rotation {
                Rotation::Left => {
                    if led.is_set_high() {
                        buzzer_0.finished_playing = true;
                    } else {
                        buzzer_0.adjust_period(20);
                    }
                    analog_value_pin25.dec();
                }
                Rotation::Right => {
                    if led.is_set_high() {
                        buzzer_0.finished_playing = false;
                    } else {
                        buzzer_0.adjust_period(-20);
                    }
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

        delay.delay_micros(1);
    }
}
