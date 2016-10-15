//! launchpad is crate for playing with the [Texas Instruments Stellaris
//! Launchpad](http://www.ti.com/tool/ek-lm4f120xl) (not to be confused with
//! the older MSP430 Launchpad).
//!
//! It's very much a work in progress, but so far the UART, SysTick and GPIO
//! seem to work. I'm gradually trying to follow the example set by japaric in
//! his [F3 crate](https://github.com/japaric/f3) for the STM32F3 Discovery
//! Board.

#![allocator]
#![crate_type="staticlib"]
#![feature(allocator)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![warn(dead_code)]

// ****************************************************************************
//
// Crates
//
// ****************************************************************************

extern crate compiler_builtins;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate lazy_static;
extern crate linked_list_allocator;
extern crate r0;
extern crate rlibc;
extern crate spin;
extern crate volatile_register;

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod board;
pub mod common;
pub mod cpu;

pub use cpu::lm4f120h5qr::systick::delay;

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

// None

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
