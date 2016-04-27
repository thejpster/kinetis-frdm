//! A GPIO library for the TI LM4F120H5QR

#![allow(dead_code)]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_store, volatile_load};
use core::ptr::Unique;
use lm4f120h5qr::registers;

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
    let mut registers = get_port_registers(pinport);
    let mask = get_pin_mask(pinport);
    unsafe {
        // register_map[port]->DATA[mask] = level ? 0xFF : 0x00;
        match level {
            Level::Low => registers.get_mut().data_mask[mask] = 0,
            Level::High => registers.get_mut().data_mask[mask] = 0xFF,
        }
    }
}

/// Read the level of an input pin
pub fn read(pinport: PinPort) -> Level {
    let mut registers = get_port_registers(pinport);
    let mask = get_pin_mask(pinport);
    unsafe {
        let reg: usize = registers.get_mut().data_mask[mask];
        if reg == 0 {
            Level::Low
        } else {
            Level::High
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
        let mut reg = volatile_load(registers::SYSCTL_RCGCGPIO_R);
        if (reg & mask) == 0 {
            reg |= mask;
            volatile_store(registers::SYSCTL_RCGCGPIO_R, reg);
            // Wait for module to settle
            while (volatile_load(registers::SYSCTL_RCGCGPIO_R) & mask) == 0 {
                asm!("NOP");
            }
        }
    }
}

fn force_gpio_periph(pinport: PinPort) {
    let mut registers = get_port_registers(pinport);
    let mask = get_pin_mask(pinport);
    let pctl_mask = get_pctl_mask(pinport);
    unsafe {
        registers.get_mut().afsel &= !mask;
        registers.get_mut().pctl &= !pctl_mask;
    }
}

fn make_input(pinport: PinPort) {
    enable_port(pinport);
    force_gpio_periph(pinport);
    let mask = get_pin_mask(pinport);
    let mut registers = get_port_registers(pinport);
    unsafe {
        if pinport == PinPort::PortF(Pin::Pin0) {
            // The GPIO for button one is multiplexed with NMI so we
            // have to 'unlock' it before we can use it
            registers.get_mut().lock = registers::GPIO_LOCK_KEY;
            registers.get_mut().cr |= mask;
            registers.get_mut().lock = 0;
        }
        registers.get_mut().den |= mask;
        registers.get_mut().dir &= mask;
    }
    force_gpio_periph(pinport);
}

fn make_input_pullup(pinport: PinPort) {
    make_input(pinport);
    let mask = get_pin_mask(pinport);
    let mut registers = get_port_registers(pinport);
    unsafe {
        registers.get_mut().dr2r |= mask;
        registers.get_mut().pur |= mask;
    }
}

fn make_input_pulldown(pinport: PinPort) {
    make_input(pinport);
    let mask = get_pin_mask(pinport);
    let mut registers = get_port_registers(pinport);
    unsafe {
        registers.get_mut().dr2r |= mask;
        registers.get_mut().pur &= !mask;
    }
}

fn make_output(pinport: PinPort, level: Level) {
    enable_port(pinport);
    force_gpio_periph(pinport);
    let mut registers = get_port_registers(pinport);
    let mask = get_pin_mask(pinport);
    unsafe {
        match level {
            Level::Low => registers.get_mut().data_mask[mask] = 0,
            Level::High => registers.get_mut().data_mask[mask] = 0xFF,
        }
        registers.get_mut().dir |= mask;
        registers.get_mut().den |= mask;
    }
}

fn get_port_registers(port: PinPort) -> Unique<registers::GpioRegisters> {
    match port {
        PinPort::PortA(_) => unsafe { Unique::new(registers::GPIO_PORTA_DATA_BITS_R as *mut _) },
        PinPort::PortB(_) => unsafe { Unique::new(registers::GPIO_PORTB_DATA_BITS_R as *mut _) },
        PinPort::PortC(_) => unsafe { Unique::new(registers::GPIO_PORTC_DATA_BITS_R as *mut _) },
        PinPort::PortD(_) => unsafe { Unique::new(registers::GPIO_PORTD_DATA_BITS_R as *mut _) },
        PinPort::PortE(_) => unsafe { Unique::new(registers::GPIO_PORTE_DATA_BITS_R as *mut _) },
        PinPort::PortF(_) => unsafe { Unique::new(registers::GPIO_PORTF_DATA_BITS_R as *mut _) },
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
