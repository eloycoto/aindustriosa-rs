#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(unused_imports)]

use panic_rtt_target as _panic_handler;
use rtic::app;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use core::fmt::Write;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::{
        gpio::{gpioa::PA0, gpioc::PC6, Alternate, Edge, Input, Output, Pin, PushPull},
        prelude::*,
    };
    use systick_monotonic::{fugit::Duration, Systick};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<'C', 13, Output<PushPull>>,
        pin: Pin<'A', 0, Input>,
        finish: Pin<'B', 6, Input>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type Tonic = Systick<1000>;

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let rcc = ctx.device.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        let gpioa = ctx.device.GPIOA.split();
        let mut pin = gpioa.pa0.into_pull_up_input();
        let mut sys_cfg = ctx.device.SYSCFG.constrain();
        pin.make_interrupt_source(&mut sys_cfg);
        pin.enable_interrupt(&mut ctx.device.EXTI);
        pin.trigger_on_edge(&mut ctx.device.EXTI, Edge::Falling);

        let gpiob = ctx.device.GPIOB.split();
        let mut finish = gpiob.pb6.into_input();
        finish.make_interrupt_source(&mut sys_cfg);
        finish.trigger_on_edge(&mut ctx.device.EXTI, Edge::Rising);
        finish.enable_interrupt(&mut ctx.device.EXTI);

        let mono = Systick::new(ctx.core.SYST, 48_000_000);

        blink::spawn().ok();

        (
            Shared {},
            Local { pin, led, finish },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led], priority = 4)]
    fn blink(ctx: blink::Context) {
        let count = COUNTER.swap(0, Ordering::SeqCst);
        rprintln!("{}", count);
        ctx.local.led.toggle();
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).ok();
    }

    #[task(binds = EXTI0, local = [pin])]
    fn on_exti(ctx: on_exti::Context) {
        ctx.local.pin.clear_interrupt_pending_bit();
        rprintln!("incrementing");
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    #[task(binds = EXTI9_5, priority = 1, local = [finish])]
    fn finish_handler(mut ctx: finish_handler::Context) {
        rprintln!("finish_handler_here");
        ctx.local.finish.clear_interrupt_pending_bit();
    }
}
