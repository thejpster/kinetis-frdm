//! # Timers for the LM4F120
//!

//! The Stellaris core has six 16/32-bit timers and six 32/64-bit wide timers.
//! Each timer provides two timers that can operate independently, or be
//! chained together to form a single double-width timer. The Cortex-M4 core
//! also its own separate SysTick timer. This is a 24-bit timer with its own
//! ISR.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_store, volatile_load};
use spin::Mutex;
use super::registers;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

pub enum Timer {
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Timer4,
    Timer5
}

pub enum WideTimer {
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Timer4,
    Timer5,
}

pub struct SysTick {
    ctrl: usize,
    reload: usize,
    current: usize
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

/// SysTick is a 24-bit timer
pub const SYSTICK_MAX:usize = (1 << 24) - 1;

lazy_static! {
    pub static ref SYSTICK: Mutex<SysTick> = Mutex::new(
        SysTick::new(
            registers::NVIC_ST_CTRL_R as usize,
            registers::NVIC_ST_RELOAD_R as usize,
            registers::NVIC_ST_CURRENT_R as usize,
            SYSTICK_MAX
        )
    );
}

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

impl SysTick {
    fn new(ctrl: usize, reload: usize, current: usize, max: usize) -> SysTick {
        let mut result = SysTick { ctrl: ctrl, reload: reload, current: current };
        result.set_max(max);
        result
    }

    pub fn set_max(&mut self, max: usize) {
        unsafe {
            // SysTick counts down from max to zero
            volatile_store(self.reload as *mut usize, max);
            // A write to current resets the timer
            volatile_store(self.current as *mut usize, 0);
            // Set to multi-shot mode, with interrupts off and on the system clock
            volatile_store(self.ctrl as *mut usize, registers::NVIC_ST_CTRL_ENABLE);
        }
    }

    pub fn get(&self) -> usize {
        let result = unsafe {
            volatile_load(self.current as *const usize)
        };
        result
    }

    pub fn ticks_to_usecs(ticks: usize) -> usize {
        let clock_rate = 4_000_000;
        ticks / (clock_rate / 1_000_000)
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
