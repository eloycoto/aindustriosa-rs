#![no_std]
#![no_main]

// Imports
use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_halt as _;
use stm32f4xx_hal::{dwt::DwtExt, gpio, interrupt, pac, prelude::*};

type FinishPin = gpio::PB6<gpio::Input>;
static FINISH: Mutex<RefCell<Option<FinishPin>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Program init").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    let mut syscfg = dp.SYSCFG.constrain();

    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high();

    let gpiob = dp.GPIOB.split();
    let mut finish = gpiob.pb6.into_input();
    finish.make_interrupt_source(&mut syscfg);
    finish.trigger_on_edge(&mut dp.EXTI, gpio::Edge::Rising);
    finish.enable_interrupt(&mut dp.EXTI);

    unsafe {
        cortex_m::peripheral::NVIC::unmask(finish.interrupt());
    }

    cortex_m::interrupt::free(|cs| {
        FINISH.borrow(cs).replace(Some(finish));
    });

    loop {
        delay.delay_ms(1000u16);
        led.toggle();
    }
}

#[interrupt]
fn EXTI9_5() {
    cortex_m::interrupt::free(|cs| {
        let mut hstdout = hio::hstdout().unwrap();
        writeln!(hstdout, "Trigger an interrupt").unwrap();
        let mut finish = FINISH.borrow(cs).borrow_mut();
        finish.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}
