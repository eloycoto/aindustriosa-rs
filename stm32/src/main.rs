#![no_std]
#![no_main]

// Imports
use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_halt as _;
use stm32f4xx_hal::{dwt::DwtExt, pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Hello, world!").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high();

    loop {
        delay.delay_ms(1000u16);
        led.toggle();
        writeln!(hstdout, "Leed toogle!").unwrap();
    }
}
