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
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
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
pub const SYSTICK_MAX:u32 = (1 << 24) - 1;

/// SysTick runs at 16MHz / 4 = 4MHz
pub const SYSTICK_CLOCK:u32 = 4_000_000;

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

lazy_static! {
    pub static ref SYSTICK_WRAP_COUNT:AtomicUsize = ATOMIC_USIZE_INIT;
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
    fn new(ctrl: usize, reload: usize, current: usize, max: u32) -> SysTick {
        let mut result = SysTick { ctrl: ctrl, reload: reload, current: current };
        result.set_max(max);
        result
    }

    fn set_max(&mut self, max: u32) {
        unsafe {
            // SysTick counts down from max to zero
            volatile_store(self.reload as *mut usize, max as usize);
            // A write to current resets the timer
            volatile_store(self.current as *mut usize, 0);
            // Set to multi-shot mode, with interrupts off and on the system clock
            volatile_store(self.ctrl as *mut usize, registers::NVIC_ST_CTRL_ENABLE | registers::NVIC_ST_CTRL_INTEN);
        }
    }

    fn get(&self) -> u32 {
        let result = unsafe {
            volatile_load(self.current as *const usize)
        };
        result as u32
    }

    fn since(&self, start: u32) -> u32 {
        let now = self.get();
        // SysTick counts down!
        let delta = start.wrapping_sub(now) & SYSTICK_MAX;
        delta
    }

    pub fn ticks_to_usecs(ticks: u32) -> u32 {
        ticks / (SYSTICK_CLOCK / 1_000_000)
    }

    pub fn usecs_to_ticks(usecs: u32) -> u32 {
        usecs * (SYSTICK_CLOCK / 1_000_000)
    }

    pub fn isr() {
        SYSTICK_WRAP_COUNT.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_overflows() -> usize {
        SYSTICK_WRAP_COUNT.load(Ordering::Relaxed)
    }

    pub fn get_ticks() -> u32 {
        SYSTICK.lock().get()
    }

    pub fn get_since(start: u32) -> u32 {
        SYSTICK.lock().since(start)
    }

}

/// Busy-waits for the given period.
///
/// Uses SysTick to wait the correct amount of time.
///
/// * `ms` - The period to wait, in milliseconds
pub fn delay(ms: u32) {
    // We can manage 4 seconds before SysTick wraps
    // We divide it up into seconds.
    let seconds = ms / 1000;
    let usec = (ms % 1000) * 1000;
    for _ in 0..seconds {
        delay_usec(1_000_000);
    }
    delay_usec(usec);
}

/// Busy-waits a specified number of microseconds.
/// `usec` must be less than 2**22 otherwise SysTick
/// will overflow.
pub fn delay_usec(usec: u32)
{
    let start = SysTick::get_ticks();
    let ticks = SysTick::usecs_to_ticks(usec);
    loop {
        if SysTick::get_since(start) >= ticks {
            break
        }
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
