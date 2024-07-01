#![no_std]
#![no_main]

// Imports
use core::cell::RefCell;
use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_halt as _;
use stm32f4::stm32f410::ADC1;
use stm32f4xx_hal::adc::config::{AdcConfig, Dma, Scan};
use stm32f4xx_hal::{adc::Adc, dwt::DwtExt, pac, prelude::*};

use heapless::arc_pool;

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

    let adc_config = AdcConfig::default()
        .dma(Dma::Continuous)
        .scan(Scan::Enabled);

    let adc = Adc::adc1(dp.ADC1, true, adc_config);
    let adc_ref = RefCell::new(adc);
    arc_pool!(P: RefCell<Adc<ADC1>>);

    let arc = P.alloc(adc_ref).unwrap();
    let _x: u16 = arc
        .borrow_mut()
        .read(&mut gpioa.pa0.into_analog())
        .unwrap()
        .clone();

    let _arc2 = arc.clone();

    loop {
        delay.delay_ms(1000u16);
        led.toggle();
        writeln!(hstdout, "Leed toogle!").unwrap();
    }
}
