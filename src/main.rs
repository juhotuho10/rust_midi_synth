#![no_std]
#![no_main]

use esp_hal::gpio::Pin;

use esp_backtrace as _;
use esp_hal::analog::dac::Dac;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{AnyPin, Event, Input, Io, Level, Output, Pull};
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::{HSClockSource, TimerIFace};
use esp_hal::ledc::{channel, timer, HighSpeed, LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::main;
use esp_hal::time::RateExtU32;
use log::info;

use esp_println::println;

use heapless::{Deque, LinearMap, Vec};
use midly::num::{u28, u4, u7};
use midly::{
    parse, EventIter, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind,
};

// =============================================================================================
//                                      SONG HEX
// =============================================================================================

const MIDI_DATA: &[u8] = &[
    0x4d, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x00, 0x60, 0x4d, 0x54,
    0x72, 0x6b, 0x00, 0x00, 0x00, 0x5c, 0x00, 0xff, 0x01, 0x24, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x73,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f,
    0x77, 0x77, 0x77, 0x2e, 0x62, 0x65, 0x65, 0x70, 0x62, 0x6f, 0x78, 0x2e, 0x63, 0x6f, 0x00, 0xff,
    0x51, 0x03, 0x05, 0x16, 0x15, 0x00, 0xff, 0x58, 0x04, 0x04, 0x02, 0x18, 0x08, 0x00, 0xff, 0x59,
    0x02, 0x00, 0x00, 0x00, 0xff, 0x06, 0x0a, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x53, 0x74, 0x61, 0x72,
    0x74, 0xce, 0x00, 0xff, 0x06, 0x08, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x45, 0x6e, 0x64, 0x00, 0xff,
    0x2f, 0x00, 0x4d, 0x54, 0x72, 0x6b, 0x00, 0x00, 0x0a, 0xf1, 0x00, 0xff, 0x03, 0x0e, 0x70, 0x69,
    0x74, 0x63, 0x68, 0x31, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x00, 0xb0, 0x65, 0x00,
    0x00, 0xb0, 0x64, 0x00, 0x00, 0xb0, 0x06, 0x18, 0x00, 0xb0, 0x26, 0x00, 0x00, 0xb0, 0x65, 0x7f,
    0x00, 0xb0, 0x64, 0x7f, 0x00, 0xff, 0x04, 0x0c, 0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x31, 0x00, 0xc0, 0x00, 0x00, 0xb0, 0x07, 0x64, 0x00, 0xb0, 0x0a, 0x40, 0x00,
    0x90, 0x42, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x32, 0x5a, 0x2c, 0x80, 0x42, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x32, 0x5a, 0x04, 0x90, 0x42, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00,
    0x90, 0x32, 0x5a, 0x2c, 0x80, 0x42, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x32, 0x5a, 0x34,
    0x90, 0x42, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x32, 0x5a, 0x2c, 0x80, 0x42, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x32, 0x5a, 0x34, 0x90, 0x42, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x32, 0x5a, 0x2c, 0x80, 0x42, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x32, 0x5a, 0x04,
    0x90, 0x42, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x32, 0x5a, 0x5c, 0x80, 0x42, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x32, 0x5a, 0x04, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00,
    0x90, 0x4f, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x64, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x2b, 0x5a, 0x5c,
    0x80, 0x43, 0x5a, 0x00, 0x80, 0x2b, 0x5a, 0x64, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x5c, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x34,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x34, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x43, 0x5a, 0x00, 0x80, 0x34, 0x5a, 0x34, 0x90, 0x37, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x30, 0x5a, 0x34,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x45, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x04, 0x90, 0x3e, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x5c, 0x80, 0x3e, 0x5a, 0x00, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x3d, 0x5a, 0x00, 0x90, 0x46, 0x5a, 0x00, 0x90, 0x36, 0x5a, 0x2c, 0x80, 0x3d, 0x5a, 0x00,
    0x80, 0x46, 0x5a, 0x00, 0x80, 0x36, 0x5a, 0x04, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x04,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x34, 0x5a, 0x3c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x43, 0x5a, 0x00, 0x80, 0x34, 0x5a, 0x04, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x3c, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x3c, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x04, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x51, 0x5a, 0x00,
    0x90, 0x41, 0x5a, 0x5c, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x51, 0x5a, 0x00, 0x80, 0x41, 0x5a, 0x04,
    0x90, 0x45, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x00, 0x90, 0x3e, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x00,
    0x80, 0x4d, 0x5a, 0x00, 0x80, 0x3e, 0x5a, 0x04, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00,
    0x90, 0x40, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x34,
    0x90, 0x45, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x5c, 0x80, 0x45, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x39, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x39, 0x5a, 0x04,
    0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x00, 0x90, 0x3b, 0x5a, 0x2c, 0x80, 0x41, 0x5a, 0x00,
    0x80, 0x4a, 0x5a, 0x00, 0x80, 0x3b, 0x5a, 0x04, 0x90, 0x3e, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x5c, 0x80, 0x3e, 0x5a, 0x00, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x34,
    0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x40, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x34, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x43, 0x5a, 0x00,
    0x90, 0x34, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x34, 0x5a, 0x34,
    0x90, 0x37, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x00,
    0x80, 0x40, 0x5a, 0x00, 0x80, 0x30, 0x5a, 0x34, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x04,
    0x90, 0x3e, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x3e, 0x5a, 0x00,
    0x80, 0x47, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x3d, 0x5a, 0x00, 0x90, 0x46, 0x5a, 0x00,
    0x90, 0x36, 0x5a, 0x2c, 0x80, 0x3d, 0x5a, 0x00, 0x80, 0x46, 0x5a, 0x00, 0x80, 0x36, 0x5a, 0x04,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x5c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x45, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x04, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x43, 0x5a, 0x00,
    0x90, 0x34, 0x5a, 0x3c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x34, 0x5a, 0x04,
    0x90, 0x43, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x3c, 0x80, 0x43, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00,
    0x90, 0x40, 0x5a, 0x3c, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x04,
    0x90, 0x48, 0x5a, 0x00, 0x90, 0x51, 0x5a, 0x00, 0x90, 0x41, 0x5a, 0x5c, 0x80, 0x48, 0x5a, 0x00,
    0x80, 0x51, 0x5a, 0x00, 0x80, 0x41, 0x5a, 0x04, 0x90, 0x45, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x00,
    0x90, 0x3e, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x00, 0x80, 0x3e, 0x5a, 0x04,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x34, 0x90, 0x45, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x5c, 0x80, 0x45, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04,
    0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x39, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x39, 0x5a, 0x04, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x00,
    0x90, 0x3b, 0x5a, 0x2c, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x00, 0x80, 0x3b, 0x5a, 0x04,
    0x90, 0x3e, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x3e, 0x5a, 0x00,
    0x80, 0x47, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x34, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04,
    0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04,
    0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00,
    0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c,
    0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30,
    0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x04, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x4c, 0x5a, 0x2c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x04,
    0x90, 0x35, 0x5a, 0x30, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00,
    0x90, 0x44, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x44, 0x5a, 0x04,
    0x90, 0x41, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x04,
    0x90, 0x43, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x43, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04, 0x90, 0x3c, 0x5a, 0x30, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x45, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x04, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x04, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x00, 0x90, 0x41, 0x5a, 0x00,
    0x90, 0x4a, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x04,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c, 0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4b, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x34,
    0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00, 0x90, 0x54, 0x5a, 0x5c, 0x80, 0x4d, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04, 0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00,
    0x90, 0x54, 0x5a, 0x2c, 0x80, 0x4d, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04,
    0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00, 0x90, 0x54, 0x5a, 0x5c, 0x80, 0x4d, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c, 0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4b, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x04, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x2c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x04, 0x90, 0x35, 0x5a, 0x30, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x44, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00,
    0x80, 0x40, 0x5a, 0x00, 0x80, 0x44, 0x5a, 0x04, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c,
    0x80, 0x41, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x04, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04,
    0x90, 0x3c, 0x5a, 0x30, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x04,
    0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x04, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x00, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00,
    0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x04, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04,
    0x90, 0x44, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x38, 0x5a, 0x5c, 0x80, 0x44, 0x5a, 0x00,
    0x80, 0x4b, 0x5a, 0x00, 0x80, 0x38, 0x5a, 0x34, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x00,
    0x90, 0x3a, 0x5a, 0x5c, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x00, 0x80, 0x3a, 0x5a, 0x34,
    0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x5c, 0x80, 0x40, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x34, 0x90, 0x37, 0x5a, 0x2c, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x37, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c, 0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00,
    0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00,
    0x80, 0x4b, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x04, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x2c, 0x80, 0x3c, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x04, 0x90, 0x35, 0x5a, 0x30, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x44, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00,
    0x80, 0x40, 0x5a, 0x00, 0x80, 0x44, 0x5a, 0x04, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c,
    0x80, 0x41, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x04, 0x90, 0x43, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x43, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04,
    0x90, 0x3c, 0x5a, 0x30, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x04,
    0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x04, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x00, 0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00,
    0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x04, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04,
    0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04,
    0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00,
    0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c,
    0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30,
    0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00,
    0x90, 0x37, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x48, 0x5a, 0x00, 0x90, 0x4c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x48, 0x5a, 0x00,
    0x80, 0x4c, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x34, 0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00,
    0x90, 0x54, 0x5a, 0x5c, 0x80, 0x4d, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04,
    0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00, 0x90, 0x54, 0x5a, 0x2c, 0x80, 0x4d, 0x5a, 0x00,
    0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04, 0x90, 0x4d, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x00,
    0x90, 0x54, 0x5a, 0x5c, 0x80, 0x4d, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x00, 0x80, 0x54, 0x5a, 0x04,
    0x90, 0x37, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04,
    0x90, 0x4c, 0x5a, 0x00, 0x90, 0x4f, 0x5a, 0x2c, 0x80, 0x4c, 0x5a, 0x00, 0x80, 0x4f, 0x5a, 0x04,
    0x90, 0x4b, 0x5a, 0x00, 0x90, 0x4e, 0x5a, 0x00, 0x90, 0x37, 0x5a, 0x2c, 0x80, 0x4b, 0x5a, 0x00,
    0x80, 0x4e, 0x5a, 0x00, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x4a, 0x5a, 0x00, 0x90, 0x4d, 0x5a, 0x2c,
    0x80, 0x4a, 0x5a, 0x00, 0x80, 0x4d, 0x5a, 0x04, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x30,
    0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x90, 0x47, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x47, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x04, 0x80, 0x3c, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x4c, 0x5a, 0x2c, 0x80, 0x3c, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x4c, 0x5a, 0x04,
    0x90, 0x35, 0x5a, 0x30, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x00, 0x90, 0x40, 0x5a, 0x00,
    0x90, 0x44, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x44, 0x5a, 0x04,
    0x90, 0x41, 0x5a, 0x00, 0x90, 0x45, 0x5a, 0x2c, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x45, 0x5a, 0x04,
    0x90, 0x43, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00, 0x90, 0x3c, 0x5a, 0x2c, 0x80, 0x43, 0x5a, 0x00,
    0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x04, 0x90, 0x3c, 0x5a, 0x30, 0x80, 0x3c, 0x5a, 0x00,
    0x90, 0x45, 0x5a, 0x2c, 0x80, 0x45, 0x5a, 0x04, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x2c, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x35, 0x5a, 0x00,
    0x90, 0x35, 0x5a, 0x04, 0x80, 0x35, 0x5a, 0x00, 0x90, 0x35, 0x5a, 0x00, 0x90, 0x41, 0x5a, 0x00,
    0x90, 0x4a, 0x5a, 0x2c, 0x80, 0x35, 0x5a, 0x00, 0x80, 0x41, 0x5a, 0x00, 0x80, 0x4a, 0x5a, 0x04,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04, 0x90, 0x44, 0x5a, 0x00, 0x90, 0x4b, 0x5a, 0x00,
    0x90, 0x38, 0x5a, 0x5c, 0x80, 0x44, 0x5a, 0x00, 0x80, 0x4b, 0x5a, 0x00, 0x80, 0x38, 0x5a, 0x34,
    0x90, 0x41, 0x5a, 0x00, 0x90, 0x4a, 0x5a, 0x00, 0x90, 0x3a, 0x5a, 0x5c, 0x80, 0x41, 0x5a, 0x00,
    0x80, 0x4a, 0x5a, 0x00, 0x80, 0x3a, 0x5a, 0x34, 0x90, 0x40, 0x5a, 0x00, 0x90, 0x48, 0x5a, 0x00,
    0x90, 0x3c, 0x5a, 0x5c, 0x80, 0x40, 0x5a, 0x00, 0x80, 0x48, 0x5a, 0x00, 0x80, 0x3c, 0x5a, 0x34,
    0x90, 0x37, 0x5a, 0x2c, 0x80, 0x37, 0x5a, 0x04, 0x90, 0x37, 0x5a, 0x5c, 0x80, 0x37, 0x5a, 0x04,
    0x90, 0x30, 0x5a, 0x5c, 0x80, 0x30, 0x5a, 0x04, 0xff, 0x2f, 0x00,
];

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
        let micros_per_min = 60_000_000;
        self.bpm = (micros_per_min / tempo) as u16;
    }
}

// =============================================================================================
//                                      SONG PLAYER
// =============================================================================================

struct SongPlayer<'a> {
    sound_data: InstrumentSounds,
    free_buzzers: Deque<SoundBuzzer<'a>, 16>,
    taken_buzzers: LinearMap<(u4, u7), SoundBuzzer<'a>, 16>,
}

impl<'a> SongPlayer<'a> {
    fn new(buzzers: Deque<SoundBuzzer<'a>, 16>) -> Self {
        SongPlayer {
            sound_data: InstrumentSounds::new(),
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

    fn delta_to_micros(delta_ticks: u32, meta_data: &SongMetaData) -> u64 {
        (delta_ticks as u64 * meta_data.tempo as u64) / meta_data.ticks_per_quarter as u64
    }

    fn play_song(&mut self, metadata: &mut SongMetaData, song_events: EventIter) {
        for event in song_events.flatten() {
            let TrackEvent { delta, kind } = event;

            let arbitrary_len = 2.7;

            let mut delta_time =
                (Self::delta_to_micros(delta.as_int(), metadata) as f32 * arbitrary_len) as u64;
            println!("{}", delta_time);
            while delta_time > 0 {
                self.play_buzzers();
                delta_time -= 1;
            }
            self.free_buzzers();

            self.match_music_events(metadata, kind);
        }
    }

    fn match_music_events(&mut self, metadata: &mut SongMetaData, event_kind: TrackEventKind) {
        match event_kind {
            TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel } => {
                    println!("taken buzzers len: {:?}", self.taken_buzzers.len());
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
                MetaMessage::Tempo(u24) => todo!(),
                MetaMessage::SmpteOffset(smpte_time) => todo!(),
                MetaMessage::TimeSignature(a, b, c, d) => metadata.time_signature = [a, b, c, d],
                MetaMessage::KeySignature(key, sharp) => metadata.key = (key, sharp),

                MetaMessage::MidiChannel(u4) => println!("not implemented: num midi channels"),
                MetaMessage::MidiPort(u7) => println!("not implemented: num midi ports"),
                MetaMessage::TrackNumber(_) => println!("not implemented: track number"),
                MetaMessage::Text(items) => println!("not implemented: text"),
                MetaMessage::TrackName(items) => println!("not implemented: track name"),

                _ => {}
            },
            TrackEventKind::SysEx(_) => {}
            TrackEventKind::Escape(_) => {}
        }
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
            profiles: [SoundProfile { frequency: 4000 }; 128],
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
}

impl SoundBuzzer<'_> {
    fn new(pin: AnyPin, period_micros: u16) -> Self {
        Self {
            buzzer_pin: Output::new(pin, Level::Low),
            period_micros,
            half_period_micros: (period_micros / 2),
            current_micros: 0,
            playing_note: None,
            pin_state: false,
            finished_playing: true,
        }
    }

    fn reset(&mut self) {
        self.current_micros = 0;
        self.buzzer_pin.set_low();
        self.playing_note = None;
        self.pin_state = false;
        self.finished_playing = true;
    }

    fn set_frquency_from_note(&mut self) {
        if let Some(note) = &self.playing_note {
            self.period_micros = note.sound.frequency - ((6000 / 128) * (note.key as u16 - 64));
        }
    }

    fn update(&mut self) {
        // todo: maybe optimize this, this taking time = 10000% more delay in songs
        // TODO: when changing the frequency to be from hz, remake this

        self.current_micros += 1;
        let new_state = self.current_micros > self.half_period_micros;
        if !self.finished_playing && new_state != self.pin_state {
            self.buzzer_pin.toggle();
            self.pin_state = !self.pin_state;
        } else if self.current_micros > self.period_micros {
            self.current_micros = 0;
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
    let mut track_iter = track_iter.flatten();
    println!("{:?}", header);
    let meta_info = track_iter.next().unwrap();

    let track = track_iter.next().unwrap();

    let mut track_meta_data = SongMetaData::new(header, meta_info);

    println!("{:?}", track_meta_data);

    // println!("track music data");
    // for track_event in track.clone() {
    //     println!("{:?}", track_event);
    // }

    // ---------- set up pins ----------

    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    // roatry encoder input pins
    let clk = Input::new(peripherals.GPIO5, Pull::Up);
    let dt = Input::new(peripherals.GPIO13, Pull::Up);
    let sw = Input::new(peripherals.GPIO12, Pull::Up);

    // ---------- set up analog DAC pins ----------

    let mut dac_25 = Dac::new(peripherals.DAC1, peripherals.GPIO25);

    // ---------- set up PWM for driving buzzer ----------

    // let mut ledc = Ledc::new(peripherals.LEDC);
    // ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    //
    // let mut lstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    // lstimer0
    //     .configure(timer::config::Config {
    //         duty: timer::config::Duty::Duty3Bit,
    //         clock_source: timer::HSClockSource::APBClk,
    //         frequency: 4u32.MHz(),
    //     })
    //     .unwrap();
    //
    // let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    // channel0
    //     .configure(channel::config::Config {
    //         timer: &lstimer0,
    //         duty_pct: 100,
    //         pin_config: channel::config::PinConfig::PushPull,
    //     })
    //     .unwrap();
    //
    // loop {
    //     // Set up a breathing LED: fade from off to on over a second, then
    //     // from on back off over the next second.  Then loop.
    //
    //     channel0.start_duty_fade(10, 100, 1).unwrap();
    //     while channel0.is_duty_fade_running() {}
    //     channel0.start_duty_fade(100, 10, 1).unwrap();
    //     while channel0.is_duty_fade_running() {}
    // }

    // ---------- set baseline states ----------

    let buzzer_1 = SoundBuzzer::new(peripherals.GPIO14.degrade(), 2000);
    let buzzer_2 = SoundBuzzer::new(peripherals.GPIO27.degrade(), 2000);
    let buzzer_3 = SoundBuzzer::new(peripherals.GPIO16.degrade(), 2000);

    let mut analog_value_pin25 = Analog8::default();
    let mut buzzer_queue: Deque<SoundBuzzer, 16> = Deque::new();
    let _ = buzzer_queue.push_back(buzzer_1);
    //let _ = buzzer_queue.push_back(buzzer_2);
    //let _ = buzzer_queue.push_back(buzzer_3);

    let mut song_player = SongPlayer::new(buzzer_queue);

    dac_25.write(analog_value_pin25.value);

    // last states for rotary encode pins
    let mut last_clk_state = clk.is_high();
    let mut last_dt_state = dt.is_high();
    let mut last_sw_state = sw.is_low();

    let mut buzzer_0 = SoundBuzzer::new(peripherals.GPIO26.degrade(), 2000);
    buzzer_0.finished_playing = false;
    //buzzer_queue.push_back(buzzer_0);

    // todo: add a self healing meachanism that tries to catch up / slow down to get the correct beat

    song_player.play_song(&mut track_meta_data, track);

    println!("song over");

    loop {
        // current states
        let current_clk_state = clk.is_high();
        let current_dt_state = dt.is_high();
        let current_sw_state = sw.is_low();

        // pin logic
        if sw.is_low() && current_sw_state != last_sw_state {
            led.toggle();

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
