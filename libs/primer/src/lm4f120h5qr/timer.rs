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

// None

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

// ****************************************************************************
//
// Public Data
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
// Private Data
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
