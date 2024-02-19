#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use embedded_alloc::Heap;
use panic_halt as _;
use prost::Message;
use stm32f4xx_hal::{dwt::DwtExt, pac, prelude::*};
extern crate alloc;

use alloc::vec::Vec;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chrono {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(uint32, tag = "2")]
    pub time: u32,
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Program init").unwrap();

    let chrono = Chrono {
        id: 1,
        time: 12123123,
    };
    writeln!(hstdout, "Chrone Len: {}", chrono.encoded_len()).unwrap();
    let mut dst = Vec::with_capacity(chrono.encoded_len());
    chrono.encode(&mut dst).unwrap();
    writeln!(hstdout, "DST: {:?}", dst).unwrap();
    drop(chrono);

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high();

    writeln!(hstdout, "start loop").unwrap();
    loop {
        delay.delay_ms(10000u16);

        led.toggle();
    }
}
