//! A blinky-LED example application
//! This example uses Primer, a library for simple bare-metal ARM programming.

#![no_std]
#![no_main]
#![feature(alloc, collections)]
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
use primer::lm4f120h5qr::{uart, systick};

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
    loop {
        let delta = systick::get_since(ticks_last);
        ticks_last = systick::get_ticks();
        writeln!(uart,
                 "Hello, world! Loops = {}, elapsed = {}, run_time = {}",
                 loops,
                 systick::ticks_to_usecs(delta),
                 systick::run_time_us() as u32)
            .unwrap();
        loops = loops + 1;
        launchpad::led_on(launchpad::Led::Blue);
        primer::delay(500);
        launchpad::led_off(launchpad::Led::Blue);
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
