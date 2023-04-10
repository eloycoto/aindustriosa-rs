#![no_std]
#![no_main]

// Imports
use bme280::BME280;
use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_halt as _;
use stm32f4xx_hal::{dwt::DwtExt, i2c::Mode, pac, prelude::*};

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

    let gpiob = dp.GPIOB.split();

    // starting i2c section
    let scl = gpiob.pb6;
    let sda = gpiob.pb9;
    let i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    // driver section
    let mut bme280 = BME280::new_primary(i2c, delay);
    match bme280.init() {
        Err(e) => writeln!(hstdout, "ERROR {:?}", e).unwrap(),
        Ok(()) => writeln!(hstdout, "BME OK").unwrap(),
    }
    loop {
        delay.delay_ms(1000u16);
        led.toggle();
        writeln!(hstdout, "Leed toogle!").unwrap();

        match bme280.measure() {
            Err(e) => writeln!(hstdout, "ERROR READING {:?}", e).unwrap(),
            Ok(m) => {
                writeln!(hstdout, "Relative Humidity = {}%", m.humidity).unwrap();
                writeln!(hstdout, "Pressure = {} pascals", m.pressure).unwrap();
                writeln!(hstdout, "Temperature = {} deg C", m.temperature).unwrap();
            }
        };
    }
}
