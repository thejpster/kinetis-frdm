//! A blinky-LED example application
//! This example uses kinetis-rs.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate kinetis_frdm;
// Need head xargo and a nightly after 2016-12-05 to enable this
// extern crate alloc;
// #[macro_use]
// extern crate collections;
extern crate cortex_m;

use kinetis_frdm::board;
use cortex_m::asm::nop;

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

pub static mut USELESS_GLOBAL_VALUE: usize = 0x123456;

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************


pub fn delay(count: u32) {
    for _ in 0..count {
        unsafe { asm!("nop" :::: "volatile") }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    // let mut test = collections::Vec::new();
    // test.push(1);
    // test.push(1);
    // test.push(1);
    let delay_time:u32 = 10_000;
    loop {
        board::led_off(board::Led::Blue);
        delay(delay_time);
        board::led_on(board::Led::Blue);
        delay(delay_time);
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
