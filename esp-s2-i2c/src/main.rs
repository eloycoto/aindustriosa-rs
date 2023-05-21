#![no_std]
#![no_main]

use bme280::i2c::BME280;
use core::cell::RefCell;
use embedded_hal_bus::i2c::RefCellDevice;
use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
    timer::TimerGroup, Delay, Rtc,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    println!("Init i2c");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio35,
        io.pins.gpio36,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let i2c_ref = RefCell::new(i2c);
    let i2c_bus = RefCellDevice::new(&i2c_ref);
    let mut bme280 = BME280::new_primary(i2c_bus);

    loop {
        match bme280.measure(&mut Delay::new(&clocks)) {
            Err(e) => println!("ERROR READING {:?}", e),
            Ok(m) => {
                println!("Relative Humidity = {}%", m.humidity);
                println!("Pressure = {} pascals", m.pressure);
                println!("Temperature = {} deg C", m.temperature);
            }
        };
    }
}
