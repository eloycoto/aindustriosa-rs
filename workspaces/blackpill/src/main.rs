#![no_std]
#![no_main]

// Imports
use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_halt as _;
use stm32f4xx_hal::{dwt::DwtExt, pac, prelude::*};

use libs::Motor;

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Program init").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high();

    let direction_pin = gpioa.pa6.into_push_pull_output();
    let action_pin = gpioa.pa7.into_push_pull_output();

    let mut motor = Motor::new(direction_pin, action_pin);
    motor.forward();

    writeln!(hstdout, "start loop").unwrap();
    loop {
        delay.delay_ms(10000u16);

        led.toggle();
    }
}
