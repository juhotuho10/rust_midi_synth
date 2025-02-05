#![no_std]
#![no_main]
use esp32_hal::adc::{AdcConfig, Attenuation, ADC, ADC2};
use esp32_hal::dac::DAC1;
use esp32_hal::prelude::*;
use esp32_hal::xtensa_lx_rt::entry;
use esp_println::println;
use panic_halt as _;

use esp32_hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, timer::TimerGroup, Delay, Rtc,
};

enum Rotation {
    Left,
    Right,
}

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
    let clk = io.pins.gpio5.into_pull_up_input();
    let dt = io.pins.gpio13.into_pull_up_input();
    let sw = io.pins.gpio12.into_pull_up_input();

    // ---------- set up analog ADC pin ----------

    //let analog = peripherals.SENS.split();

    //DAC1::dac(peripherals.AES.start(), io.pins.gpio25.into_analog());

    //let mut adc1_config = AdcConfig::new();

    //let mut pin25 =
    //    adc1_config.enable_pin(io.pins.gpio25.into_analog(), Attenuation::Attenuation11dB);

    //let mut adc2 = ADC::<ADC2>::adc(analog.adc2, adc1_config).unwrap();

    // ---------- set up analog DAC pin ----------

    let dac_pin = io.pins.gpio25.into_analog();
    let mut dac_25 = DAC1::dac(peripherals.AES, dac_pin).unwrap();

    // ---------- set baseline states ----------

    let mut analog_value_pin25 = Analog8::default();
    dac_25.write(analog_value_pin25.value);

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
                Rotation::Left => analog_value_pin25.dec(),
                Rotation::Right => analog_value_pin25.inc(),
            }

            println!("{}", analog_value_pin25.value);
            dac_25.write(analog_value_pin25.value);
        }

        // reset current states
        last_dt_state = current_dt_state;
        last_sw_state = current_sw_state;
        last_clk_state = current_clk_state;

        delay.delay_ms(1u32);
    }
}
