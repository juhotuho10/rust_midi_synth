#![no_std]
#![no_main]
use esp32_hal::prelude::*;
use esp32_hal::xtensa_lx_rt::entry;
use panic_halt as _;

use esp32_hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, timer::TimerGroup, Delay, Rtc,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    // ---------- set up clock ----------
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    wdt.disable();
    rtc.rwdt.disable();

    // ---------- set up pins ----------

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = io.pins.gpio2.into_push_pull_output();

    let mut pin0 = io.pins.gpio0.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    loop {
        led.set_high().unwrap();
        pin0.set_high().unwrap();
        delay.delay_ms(300u32);
        led.set_low().unwrap();
        pin0.set_low().unwrap();
        delay.delay_ms(300u32);
    }
}
