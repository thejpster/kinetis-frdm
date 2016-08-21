//! Implements support for the LM4F120 UARTs

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use super::registers as reg;
use super::gpio;
use super::pll;
use common::volatile::VolatileStruct;
use common;
use core;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// This chip has 8 UARTs
#[derive(PartialEq, Clone, Copy)]
pub enum UartId {
    Uart0,
    Uart1,
    Uart2,
    Uart3,
    Uart4,
    Uart5,
    Uart6,
    Uart7,
}

/// Controls a single UART
/// Only supports 8/N/1 - who needs anything else?
pub struct Uart {
    id: UartId,
    baud: u32,
    nl_mode: NewlineMode,
    reg: &'static mut reg::UartRegisters,
}

/// writeln!() emits LF chars, so this is useful
/// if you're writing text with your UART
#[derive(PartialEq, Clone, Copy)]
pub enum NewlineMode {
    Binary,
    SwapLFtoCRLF,
}

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

impl Uart {
    pub fn new(id: UartId, baud: u32, nl_mode: NewlineMode) -> Uart {
        let mut uart = Uart {
            id: id,
            baud: baud,
            nl_mode: nl_mode,
            reg: get_uart_registers(id),
        };
        uart.init();
        uart
    }

    fn init(&mut self) -> () {
        // Do GPIO pin muxing
        gpio::enable_uart(self.id);
        // Enable UART module in RCGUART register p306
        unsafe {
            self.enable_uart();

            // Disable UART and all features
            self.reg.ctl.write(0);
            // Calculate the baud rate values
            // baud_div = CLOCK_RATE / (16 * baud_rate);
            // baud_int = round(baud_div * 64)
            let baud_int: u32 = (((pll::get_clock_hz() * 8) / self.baud) + 1) / 2;
            // Store the upper and lower parts of the divider
            self.reg.ibrd.write((baud_int / 64) as usize);
            self.reg.fbrd.write((baud_int % 64) as usize);
            // Set the UART Line Control register value
            // 8N1 + FIFO enabled
            self.reg.lcrh.write(reg::UART_LCRH_WLEN_8 | reg::UART_LCRH_FEN);
            // Clear the flags
            self.reg.rf.write(0);
            // Enable
            self.reg.ctl.write(reg::UART_CTL_RXE | reg::UART_CTL_TXE |
                           reg::UART_CTL_UARTEN);
        }
    }

    unsafe fn enable_uart(&mut self) {
        common::read_set_write_settle(reg::SYSCTL_RCGCUART_R, match self.id {
            UartId::Uart0 => reg::SYSCTL_RCGCUART_R0,
            UartId::Uart1 => reg::SYSCTL_RCGCUART_R1,
            UartId::Uart2 => reg::SYSCTL_RCGCUART_R2,
            UartId::Uart3 => reg::SYSCTL_RCGCUART_R3,
            UartId::Uart4 => reg::SYSCTL_RCGCUART_R4,
            UartId::Uart5 => reg::SYSCTL_RCGCUART_R5,
            UartId::Uart6 => reg::SYSCTL_RCGCUART_R6,
            UartId::Uart7 => reg::SYSCTL_RCGCUART_R7,
        });
    }

    fn putc(&mut self, value: u8) {
        unsafe {
            while (self.reg.rf.read() & reg::UART_FR_TXFF) != 0 {
                asm!("NOP");
            }
            self.reg.data.write(value as usize);
        }
    }

    /// Attempts to read from the UART. Returns 'None'
    /// if the FIFO is empty, or 'Some(octet)'.
    pub fn read_single(&mut self) -> Option<u8> {
        if (self.reg.rf.read() & reg::UART_FR_RXFE) != 0 {
            None
        } else {
            Some(self.reg.data.read() as u8)
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.nl_mode {
            NewlineMode::Binary => {
                for byte in s.bytes() {
                    self.putc(byte)
                }
            }
            NewlineMode::SwapLFtoCRLF => {
                for byte in s.bytes() {
                    if byte == 0x0A {
                        // Prefix every \n with a \r
                        self.putc(0x0D)
                    }
                    self.putc(byte)
                }
            }
        }
        Ok(())
    }
}

/// Called when UART 0 interrupt fires
pub unsafe extern "C" fn uart0_isr() {

}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Get a Unique<> wrapped pointer to the UART control registers in the chip.
/// We use the Unique<> wrapper to make it easier to store the pointer in our
/// object.
fn get_uart_registers(uart: UartId) -> &'static mut reg::UartRegisters {
    match uart {
        UartId::Uart0 => unsafe { reg::UartRegisters::from_ptr(reg::UART0_DR_R as *mut _) },
        UartId::Uart1 => unsafe { reg::UartRegisters::from_ptr(reg::UART1_DR_R as *mut _) },
        UartId::Uart2 => unsafe { reg::UartRegisters::from_ptr(reg::UART2_DR_R as *mut _) },
        UartId::Uart3 => unsafe { reg::UartRegisters::from_ptr(reg::UART3_DR_R as *mut _) },
        UartId::Uart4 => unsafe { reg::UartRegisters::from_ptr(reg::UART4_DR_R as *mut _) },
        UartId::Uart5 => unsafe { reg::UartRegisters::from_ptr(reg::UART5_DR_R as *mut _) },
        UartId::Uart6 => unsafe { reg::UartRegisters::from_ptr(reg::UART6_DR_R as *mut _) },
        UartId::Uart7 => unsafe { reg::UartRegisters::from_ptr(reg::UART7_DR_R as *mut _) },
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
