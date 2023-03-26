#![no_std]
#![no_main]

// Imports
use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use embedded_storage::nor_flash::NorFlash;
use embedded_storage::nor_flash::ReadNorFlash;
use panic_halt as _;
use stm32f4xx_hal::{
    dwt::DwtExt,
    flash::{FlashExt, LockedFlash, UnlockedFlash},
    gpio, interrupt, pac,
    prelude::*,
};

type FinishPin = gpio::PB6<gpio::Input>;
static FINISH: Mutex<RefCell<Option<FinishPin>>> = Mutex::new(RefCell::new(None));

enum FlashValues {
    LAPS,
    FOO,
}

impl FlashValues {
    fn location(&self) -> u32 {
        match *self {
            FlashValues::LAPS => 128 * 1024,
            FlashValues::FOO => 130 * 1024,
        }
    }
}

fn write_to_flash(mut flash: UnlockedFlash, location: FlashValues, val: u8) {
    let buf = [val];
    NorFlash::write(&mut flash, location.location(), &buf).unwrap();
}

fn read_from_flash(mut flash: UnlockedFlash, location: FlashValues) -> u8 {
    let mut res = [0u8; 1];
    ReadNorFlash::read(&mut flash, location.location(), &mut res).unwrap();
    return res[0];
}

fn erase_flash(mut flash: UnlockedFlash) {
    NorFlash::erase(&mut flash, 128 * 1024, 256 * 1024).unwrap();
}

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Program init").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = LockedFlash::new(dp.FLASH);

    erase_flash(flash.unlocked());
    write_to_flash(flash.unlocked(), FlashValues::LAPS, 12);

    let val = read_from_flash(flash.unlocked(), FlashValues::LAPS);
    writeln!(hstdout, "flash value {}", val).unwrap();

    write_to_flash(flash.unlocked(), FlashValues::FOO, 3);
    let ooo = read_from_flash(flash.unlocked(), FlashValues::FOO);
    writeln!(hstdout, "flash value FOO:: {}", ooo).unwrap();

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
    writeln!(hstdout, "start loop").unwrap();
    loop {
        delay.delay_ms(10000u16);

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
