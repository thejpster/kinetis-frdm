//! Modules common to all targets.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod memory;
pub mod startup;
pub mod builtins;
pub mod volatile;

use core::intrinsics::{volatile_store, volatile_load};
use core::ops::{BitOrAssign, BitAndAssign, Not};

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

/// Performs a volatile_store on the ptr, then repeatedly reads *ptr until it
/// equals the given value. Useful for writing to Run-time Clock Gating
/// Control registers on a Tiva-C/LM4F120.
pub unsafe fn write_settle<T: PartialEq + Copy>(ptr: *mut T, value: T) {
    volatile_store(ptr, value);
    // Wait for value to settle
    while volatile_load(ptr) != value {
       asm!("NOP");
    }
}

/// Performs a read modify write on the ptr, then repeatedly reads *ptr until
/// it equals the given value. Useful for writing to Run-time Clock Gating
/// Control registers on a Tiva-C/LM4F120.
pub unsafe fn read_set_write_settle<T: PartialEq + Copy + BitOrAssign>(ptr: *mut T, set_bits: T) {
    let mut t:T = volatile_load(ptr);
    t |= set_bits;
    write_settle(ptr, t);
}

/// Performs a read modify write on the ptr, then repeatedly reads *ptr until
/// it equals the given value. Useful for writing to Run-time Clock Gating
/// Control registers on a Tiva-C/LM4F120. ANDs the ! of clear_bits.
pub unsafe fn read_clear_write_settle<T: PartialEq + Copy + BitAndAssign + Not<Output=T>>(ptr: *mut T, clear_bits: T) {
    let mut t:T = volatile_load(ptr);
    t &= !clear_bits;
    write_settle(ptr, t);
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
