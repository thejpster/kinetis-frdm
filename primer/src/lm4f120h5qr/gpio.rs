//! A GPIO library for the TI LM4F120H5QR

#![allow(dead_code)]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_store, volatile_load};
use super::registers;
use super::uart::UartId;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// Describes a pin within a port
/// This chip has 8 pins per port.
#[derive(PartialEq, Clone, Copy)]
pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
}

/// Describes a Port and a single pin within it
#[derive(PartialEq, Clone, Copy)]
pub enum PinPort {
    PortA(Pin),
    PortB(Pin),
    PortC(Pin),
    PortD(Pin),
    PortE(Pin),
    PortF(Pin),
}

/// Describes a pin's direction
#[derive(PartialEq, Clone, Copy)]
pub enum PinMode {
    InputPull(Level),
    Input,
    Output,
}

/// Describes what a pin can be set to
#[derive(PartialEq, Clone, Copy)]
pub enum Level {
    High,
    Low,
}

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// init() is empty for now, but it might be needed some day.
pub fn init() {}

/// Set the direction (input or output) on a given pin in a port
pub fn set_direction(pinport: PinPort, mode: PinMode) {
    match mode {
        PinMode::InputPull(Level::High) => make_input_pullup(pinport),
        PinMode::InputPull(Level::Low) => make_input_pulldown(pinport),
        PinMode::Input => make_input(pinport),
        PinMode::Output => make_output(pinport, Level::Low),
    }
}

/// Set the output value for an output pin
pub fn set(pinport: PinPort, level: Level) {
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    match level {
        Level::Low => gpio_reg.data_mask[mask] = 0,
        Level::High => gpio_reg.data_mask[mask] = 0xFF,
    }
}

/// Read the level of an input pin
pub fn read(pinport: PinPort) -> Level {
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    if gpio_reg.data_mask[mask] == 0 {
        Level::Low
    } else {
        Level::High
    }
}

pub fn enable_uart(id: UartId) {
    match id {
        UartId::Uart0 => {
            enable_port(PinPort::PortA(Pin::Pin0));
            let gpio_reg = get_port_registers(PinPort::PortA(Pin::Pin1));
            gpio_reg.afsel |= (1 << 1) | (1 << 0);
            gpio_reg.den |= (1 << 1) | (1 << 0);
            gpio_reg.pctl &= !(registers::GPIO_PCTL_PA0_M | registers::GPIO_PCTL_PA1_M);
            gpio_reg.pctl |= registers::GPIO_PCTL_PA0_U0RX | registers::GPIO_PCTL_PA1_U0TX;
        }
        UartId::Uart1 => {
            unimplemented!();
        }
        UartId::Uart2 => {
            unimplemented!();
        }
        UartId::Uart3 => {
            unimplemented!();
        }
        UartId::Uart4 => {
            unimplemented!();
        }
        UartId::Uart5 => {
            unimplemented!();
        }
        UartId::Uart6 => {
            unimplemented!();
        }
        UartId::Uart7 => {
            unimplemented!();
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Convert a port to a bit mask
/// Port A is 1, PortF is 32
fn get_port_mask(port: PinPort) -> usize {
    match port {
        PinPort::PortA(_) => 1 << 0,
        PinPort::PortB(_) => 1 << 1,
        PinPort::PortC(_) => 1 << 2,
        PinPort::PortD(_) => 1 << 3,
        PinPort::PortE(_) => 1 << 4,
        PinPort::PortF(_) => 1 << 5,
    }
}

/// Convert a pin to a bit mask
/// Pin0 is 0, Pin7 is 128
fn get_pin_mask(pinport: PinPort) -> usize {
    let pin = match pinport {
        PinPort::PortA(ref x) => x,
        PinPort::PortB(ref x) => x,
        PinPort::PortC(ref x) => x,
        PinPort::PortD(ref x) => x,
        PinPort::PortE(ref x) => x,
        PinPort::PortF(ref x) => x,
    };
    match *pin {
        Pin::Pin0 => 1 << 0 as usize,
        Pin::Pin1 => 1 << 1 as usize,
        Pin::Pin2 => 1 << 2 as usize,
        Pin::Pin3 => 1 << 3 as usize,
        Pin::Pin4 => 1 << 4 as usize,
        Pin::Pin5 => 1 << 5 as usize,
        Pin::Pin6 => 1 << 6 as usize,
        Pin::Pin7 => 1 << 7 as usize,
    }
}

fn get_pctl_mask(pinport: PinPort) -> usize {
    let pin = match pinport {
        PinPort::PortA(ref x) => x,
        PinPort::PortB(ref x) => x,
        PinPort::PortC(ref x) => x,
        PinPort::PortD(ref x) => x,
        PinPort::PortE(ref x) => x,
        PinPort::PortF(ref x) => x,
    };
    match *pin {
        Pin::Pin0 => 7 << 0 as usize,
        Pin::Pin1 => 7 << 4 as usize,
        Pin::Pin2 => 7 << 8 as usize,
        Pin::Pin3 => 7 << 12 as usize,
        Pin::Pin4 => 7 << 16 as usize,
        Pin::Pin5 => 7 << 20 as usize,
        Pin::Pin6 => 7 << 24 as usize,
        Pin::Pin7 => 7 << 28 as usize,
    }
}

fn enable_port(port: PinPort) {
    let mask = get_port_mask(port);
    unsafe {
        let mut value = volatile_load(registers::SYSCTL_RCGCGPIO_R);
        if (value & mask) == 0 {
            value |= mask;
            volatile_store(registers::SYSCTL_RCGCGPIO_R, value);
            // Wait for module to settle
            while (volatile_load(registers::SYSCTL_RCGCGPIO_R) & mask) == 0 {
                asm!("NOP");
            }
        }
    }
}

fn force_gpio_periph(pinport: PinPort, gpio_reg: &mut registers::GpioRegisters) {
    let mask = get_pin_mask(pinport);
    let pctl_mask = get_pctl_mask(pinport);
    gpio_reg.afsel &= !mask;
    gpio_reg.pctl &= !pctl_mask;
}

fn make_input(pinport: PinPort) {
    enable_port(pinport);
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    force_gpio_periph(pinport, gpio_reg);
    if pinport == PinPort::PortF(Pin::Pin0) {
        // The GPIO for button one is multiplexed with NMI so we
        // have to 'unlock' it before we can use it
        gpio_reg.lock = registers::GPIO_LOCK_KEY;
        gpio_reg.cr |= mask;
        gpio_reg.lock = 0;
    }
    gpio_reg.den |= mask;
    gpio_reg.dir &= mask;
}

fn make_input_pullup(pinport: PinPort) {
    make_input(pinport);
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    gpio_reg.dr2r |= mask;
    gpio_reg.pur |= mask;
}

fn make_input_pulldown(pinport: PinPort) {
    make_input(pinport);
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    gpio_reg.dr2r |= mask;
    gpio_reg.pur &= !mask;
}

fn make_output(pinport: PinPort, level: Level) {
    enable_port(pinport);
    let mask = get_pin_mask(pinport);
    let gpio_reg = get_port_registers(pinport);
    force_gpio_periph(pinport, gpio_reg);
    match level {
        Level::Low => gpio_reg.data_mask[mask] = 0,
        Level::High => gpio_reg.data_mask[mask] = 0xFF,
    }
    gpio_reg.dir |= mask;
    gpio_reg.den |= mask;
}

/// Return a Unique wrapper around a pointer to the relevant Uart
/// memory-mapped IO control structure.
fn get_port_registers(port: PinPort) -> &'static mut registers::GpioRegisters {
    unsafe {
        let p_reg = match port {
            PinPort::PortA(_) => registers::GPIO_PORTA_DATA_BITS_R as *mut registers::GpioRegisters,
            PinPort::PortB(_) => registers::GPIO_PORTB_DATA_BITS_R as *mut registers::GpioRegisters,
            PinPort::PortC(_) => registers::GPIO_PORTC_DATA_BITS_R as *mut registers::GpioRegisters,
            PinPort::PortD(_) => registers::GPIO_PORTD_DATA_BITS_R as *mut registers::GpioRegisters,
            PinPort::PortE(_) => registers::GPIO_PORTE_DATA_BITS_R as *mut registers::GpioRegisters,
            PinPort::PortF(_) => registers::GPIO_PORTF_DATA_BITS_R as *mut registers::GpioRegisters,
        };
        &mut *p_reg
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
