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
use primer::lm4f120h5qr::{uart, timer, pll};

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
    let mut ticks_last: usize = timer::SYSTICK_MAX;
    loop {
        writeln!(uart, "A").unwrap();
        let ticks = timer::SYSTICK.lock().get();
        let delta = ticks_last.wrapping_sub(ticks);
        ticks_last = ticks;
        writeln!(uart, "B").unwrap();
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
        writeln!(uart, "C").unwrap();
        let mut total = 0;
        for i in v {
            total = total + i;
        }
        writeln!(uart, "Ca").unwrap();
        writeln!(uart,
                 "Hello, world! Loops = {}, total = {}, elapsed = {}",
                 loops,
                 total,
                 pll::ticks_to_usecs(delta))
            .unwrap();
        loops = loops + 1;
        writeln!(uart, "D").unwrap();
        launchpad::led_on(launchpad::Led::Red);
        writeln!(uart, "E").unwrap();
        primer::delay(250);
        writeln!(uart, "F").unwrap();
        launchpad::led_off(launchpad::Led::Red);
        writeln!(uart, "G").unwrap();
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
