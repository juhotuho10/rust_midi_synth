#![no_std]
#![no_main]
use esp32_hal::prelude::*;
use esp32_hal::xtensa_lx_rt::entry;
use panic_halt as _;

use esp32_hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, timer::TimerGroup, Delay, Rtc,
};

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
            (true, false) => return Some(Rotation::Left),
            (false, true) => return Some(Rotation::Right),
            (_, _) => return None,
        },

        (false, false) => match (current_clk, current_dt) {
            (false, true) => return Some(Rotation::Left),
            (true, false) => return Some(Rotation::Right),
            (_, _) => return None,
        },
        (_, _) => return None,
    }
}

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

    let mut delay = Delay::new(&clocks);

    // ---------- set up pins ----------

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut pin0 = io.pins.gpio0.into_push_pull_output();
    let mut led = io.pins.gpio2.into_push_pull_output();

    // roatry encoder input pins
    let clk = io.pins.gpio26.into_pull_up_input();
    let dt = io.pins.gpio1.into_pull_up_input();
    let sw = io.pins.gpio3.into_pull_up_input();

    // last states for rotary encode pins
    let mut last_clk_state = clk.is_high().unwrap();
    let mut last_dt_state = dt.is_high().unwrap();
    let mut last_sw_state = sw.is_low().unwrap();

    loop {
        // current states
        let current_clk_state = clk.is_high().unwrap();
        let current_dt_state = dt.is_high().unwrap();
        let current_sw_state = sw.is_low().unwrap();

        // pin logic
        if sw.is_low().unwrap() && current_sw_state != last_sw_state {
            led.toggle().unwrap();
        }

        if let Some(rotation) = get_knob_rotation(
            last_clk_state,
            last_dt_state,
            current_clk_state,
            current_dt_state,
        ) {
            match rotation {
                Rotation::Left => pin0.set_low().unwrap(),
                Rotation::Right => pin0.set_high().unwrap(),
            }
        }

        // reset current states
        last_dt_state = current_dt_state;
        last_sw_state = current_sw_state;
        last_clk_state = current_clk_state;

        delay.delay_ms(5u32);
    }
}
