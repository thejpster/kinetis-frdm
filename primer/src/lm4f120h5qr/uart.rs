//! Implements support for the LM4F120 UARTs

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_store, volatile_load};
use core::ptr::Unique;
use super::registers;
use super::gpio;

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
    reg: Unique<registers::UartRegisters>,
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
            let uart_reg = self.reg.get_mut();

            // Disable UART and all features
            uart_reg.ctl = 0;
            // Calculate the baud rate values
            // baud_div = CLOCK_RATE / (16 * baud_rate);
            // baud_int = round(baud_div * 64)
            let baud_int: u32 = (((66000000 * 8) / self.baud) + 1) / 2;
            // Store the upper and lower parts of the divider
            uart_reg.ibrd = (baud_int / 64) as usize;
            uart_reg.fbrd = (baud_int % 64) as usize;
            // Calculate the UART Line Control register value
            // 8N1
            uart_reg.lcrh = registers::UART_LCRH_WLEN_8;
            // Clear the flags
            uart_reg.rf = 0;
            // Enable
            uart_reg.ctl = registers::UART_CTL_RXE | registers::UART_CTL_TXE |
                           registers::UART_CTL_UARTEN;
        }
    }

    unsafe fn enable_uart(&mut self) {
        let mut reg: usize = volatile_load(registers::SYSCTL_RCGCUART_R);
        reg |= match self.id {
            UartId::Uart0 => 1 << 0,
            UartId::Uart1 => 1 << 1,
            UartId::Uart2 => 1 << 2,
            UartId::Uart3 => 1 << 3,
            UartId::Uart4 => 1 << 4,
            UartId::Uart5 => 1 << 5,
            UartId::Uart6 => 1 << 6,
            UartId::Uart7 => 1 << 7,
        };
        volatile_store(registers::SYSCTL_RCGCUART_R, reg);
    }

    fn putc(&mut self, value: u8) {
        unsafe {
            let uart_reg = self.reg.get_mut();
            while (uart_reg.rf & registers::UART_FR_TXFF) != 0 {
                asm!("NOP");
            }
            uart_reg.data = value as usize;
        }
    }
}

impl ::core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
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

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Get a Unique<> wrapped pointer to the UART control registers in the chip.
/// We use the Unique<> wrapper to make it easier to store the pointer in our
/// object.
fn get_uart_registers(uart: UartId) -> Unique<registers::UartRegisters> {
    match uart {
        UartId::Uart0 => unsafe { Unique::new(registers::UART0_DR_R as *mut _) },
        UartId::Uart1 => unsafe { Unique::new(registers::UART1_DR_R as *mut _) },
        UartId::Uart2 => unsafe { Unique::new(registers::UART2_DR_R as *mut _) },
        UartId::Uart3 => unsafe { Unique::new(registers::UART3_DR_R as *mut _) },
        UartId::Uart4 => unsafe { Unique::new(registers::UART4_DR_R as *mut _) },
        UartId::Uart5 => unsafe { Unique::new(registers::UART5_DR_R as *mut _) },
        UartId::Uart6 => unsafe { Unique::new(registers::UART6_DR_R as *mut _) },
        UartId::Uart7 => unsafe { Unique::new(registers::UART7_DR_R as *mut _) },
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
