//! A blinky-LED example application
//! This example uses Primer, a library for simple bare-metal ARM programming.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate primer;
extern crate alloc;
#[macro_use]
extern crate collections;

use core::fmt::Write;
use primer::board::launchpad;
use primer::lm4f120h5qr::{gpio, systick, timer, uart};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************


#[no_mangle]
pub extern "C" fn primer_start() {
    launchpad::init();
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    let mut loops = 0;
    let mut ticks_last = systick::SYSTICK_MAX;
    let mut t = timer::Timer::new(timer::TimerId::Timer1);
    t.enable_pwm(32768);
    gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin2));
    loop {
        let delta = systick::get_since(ticks_last);
        ticks_last = systick::get_ticks();
        writeln!(uart,
                 "Hello, world! Loops = {}, elapsed = {}, run_time = {}, timer = {}",
                 loops,
                 systick::ticks_to_usecs(delta),
                 systick::run_time_us() as u32,
                 t.get_timer())
            .unwrap();
        while let Some(ch) = uart.read_single() {
            writeln!(uart, "byte read {}", ch).unwrap();
        }
        loops = loops + 1;
        launchpad::led_off(launchpad::Led::Blue);
        t.set_pwm(8192);
        primer::delay(500);
        launchpad::led_on(launchpad::Led::Blue);
        t.set_pwm(8192 * 3);
        primer::delay(500);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
