#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl, entry, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup,
    Delay, Rtc,
};

//use esp32_hal::xtensa_lx_rt::entry; ??
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
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

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(3000u32);
        led.set_low().unwrap();
        delay.delay_ms(3000u32);
    }
}
