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
use collections::vec::Vec;
use primer::board::launchpad;
use primer::lm4f120h5qr::uart;

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
    loop {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
        let mut total = 0;
        for i in v {
            total = total + i;
        }
        writeln!(uart, "Hello, world! Loops = {}, total = {}", loops, total).unwrap();
        loops = loops + 1;
        launchpad::led_on(launchpad::Led::Red);
        primer::delay(250);
        launchpad::led_off(launchpad::Led::Red);
        primer::delay(250);
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
