//! kinetis is crate for playing with the [Freescale Kinetis
//! FRDM-KE06Z](http://www.nxp.com/products/microcontrollers-and-processors/arm-processors/kinetis-cortex-m-mcus/e-series-5v-robust-m0-plus-m4/freedom-development-platform-for-kinetis-ke06-mcus:FRDM-KE06Z)
//!
//! It's very much a work in progress.

#![crate_type="staticlib"]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(start)]
#![feature(never_type)]
#![no_std]
#![warn(dead_code)]
#![deny(missing_docs)]

// ****************************************************************************
//
// Crates
//
// ****************************************************************************

extern crate alloc_cortex_m;
extern crate compiler_builtins;
#[macro_use]
extern crate cortex_m;
pub extern crate ke06z;
extern crate r0;
extern crate rlibc;
extern crate volatile_register;

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod board;
pub mod common;

pub use ke06z as cpu;

pub use cpu::systick::delay;

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
