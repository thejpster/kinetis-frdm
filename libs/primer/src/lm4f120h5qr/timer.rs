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

use super::registers as reg;
use common;
use common::volatile::VolatileStruct;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// This chip has 6 16/32bit Timers
#[derive(PartialEq, Clone, Copy)]
pub enum TimerId {
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Timer4,
    Timer5
}

/// This chip has 6 32/64bit Wide Timers
pub enum WideTimerId {
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Timer4,
    Timer5,
}

/// Represents a 16/32-bit Timer
pub struct Timer {
    id: TimerId,
    reg: &'static mut reg::TimerRegisters,
    period: u16,
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

pub const PWM_MIN:u8 = 0;
pub const PWM_MAX:u8 = 255;

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

impl Timer {
    /// Create a new Timer object. The caller is responsible for ensuring
    /// that only one object exists per TimerId.
    pub fn new(id: TimerId) -> Timer {
        let mut t = Timer {
            id: id,
            reg: get_timer_registers(id),
            period: 0
        };
        t.init();
        t
    }

    fn init(&mut self) {
        unsafe { self.enable_clock() }
    }

    unsafe fn enable_clock(&mut self) {
        common::read_set_write_settle(reg::SYSCTL_RCGCTIMER_R, match self.id {
            TimerId::Timer0 => reg::SYSCTL_RCGCTIMER_R0,
            TimerId::Timer1 => reg::SYSCTL_RCGCTIMER_R1,
            TimerId::Timer2 => reg::SYSCTL_RCGCTIMER_R2,
            TimerId::Timer3 => reg::SYSCTL_RCGCTIMER_R3,
            TimerId::Timer4 => reg::SYSCTL_RCGCTIMER_R4,
            TimerId::Timer5 => reg::SYSCTL_RCGCTIMER_R5,
        });
    }

    pub fn enable_pwm(&mut self, period: u16) {
        self.period = period;

        {
            let mut tamr = self.reg.tamr.read();
            tamr |= reg::TIMER_TAMR_TAAMS;
            tamr &= !reg::TIMER_TAMR_TACMR;
            tamr &= !reg::TIMER_TAMR_TAMR_M;
            tamr |= reg::TIMER_TAMR_TAMR_PERIOD;
            self.reg.tamr.write(tamr);
        }

        // Default to off
        self.set_pwm(period);

        self.reg.ctl |= reg::TIMER_CTL_TAEN;
    }

    pub fn set_pwm(&mut self, on_time: u16) {
        self.reg.tailr.write(self.period as usize);
        self.reg.tamatchr.write((self.period - on_time) as usize);
    }

    pub fn get_timer(&self) -> u16 {
        self.reg.tar.read() as u16
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Get a reference to the UART control register struct in the chip.
fn get_timer_registers(timer_id: TimerId) -> &'static mut reg::TimerRegisters {
    match timer_id {
        TimerId::Timer0 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER0_CFG_R as *mut _) },
        TimerId::Timer1 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER1_CFG_R as *mut _) },
        TimerId::Timer2 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER2_CFG_R as *mut _) },
        TimerId::Timer3 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER3_CFG_R as *mut _) },
        TimerId::Timer4 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER4_CFG_R as *mut _) },
        TimerId::Timer5 => unsafe { reg::TimerRegisters::from_ptr(reg::TIMER5_CFG_R as *mut _) },
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
