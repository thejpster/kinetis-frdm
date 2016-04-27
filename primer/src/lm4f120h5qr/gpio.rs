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

/// Matches the memory-mapped GPIO register definitions for each port
struct Registers {
    data_mask: [usize; 255], // reg_t DATA[255]; /* Data - offset sets pin mask */
    data: usize, // reg_t DATA_R; /* Data register - sets all pins */
    dir: usize, // reg_t DIR_R; /* Direction */
    is: usize, // reg_t IS_R; /* Interrupt Sense */
    ibe: usize, // reg_t IBE_R; /* Interrupt Both Edges */
    iev: usize, // reg_t IEV_R; /* Interrupt Event */
    im: usize, // reg_t IM_R; /* Interrupt Mask */
    ris: usize, // const reg_t RIS_R; /* Raw Interrupt Status */
    mis: usize, // const reg_t MIS_R; /* Masked Interrupt Status */
    icr: usize, // reg_t ICR_R; /* Interrupt Clear */
    afsel: usize, // reg_t AFSEL_R; /* Alternate function Select */
    _padding: [usize; 55], // const reg_t _padding[55];
    dr2r: usize, // reg_t DR2R_R; /* 2mA drive select */
    dr4r: usize, // reg_t DR4R_R; /* 4mA drive select */
    dr8r: usize, // reg_t DR8R_R; /* 8mA drive select */
    odr: usize, // reg_t ODR_R; /* Open-drain select */
    pur: usize, // reg_t PUR_R; /* Pull-up select */
    pdr: usize, // reg_t PDR_R; /* Pull-down select */
    slr: usize, // reg_t SLR_R; /* Slew-rate control */
    den: usize, // reg_t DEN_R; /* Digital enable */
    lock: usize, // reg_t LOCK_R; /* Lock */
    cr: usize, // reg_t CR_R; /* Commit */
    amsel: usize, // reg_t AMSEL_R; /* Analog mode select */
    pctl: usize, // reg_t PCTL_R; /* Port Control */
    adcctl: usize, // reg_t ADCCTL_R; /* ADC Control */
    dmactl: usize, // reg_t DMACTL_R; /* DMA Control */
    _padding2: [usize; 678], // const reg_t _padding2[678];
    periphid4: usize, // const reg_t PeriphID4_R; /* Peripheral ID 4 */
    periphid5: usize, // const reg_t PeriphID5_R; /* Peripheral ID 5 */
    periphid6: usize, // const reg_t PeriphID6_R; /* Peripheral ID 6 */
    periphid7: usize, // const reg_t PeriphID7_R; /* Peripheral ID 7 */
    periphid0: usize, // const reg_t PeriphID0_R; /* Peripheral ID 0 */
    periphid1: usize, // const reg_t PeriphID1_R; /* Peripheral ID 1 */
    periphid2: usize, // const reg_t PeriphID2_R; /* Peripheral ID 2 */
    periphid3: usize, // const reg_t PeriphID3_R; /* Peripheral ID 3 */
    pcelld0: usize, // const reg_t PCellD0_R; /* PrimeCell ID 0 */
    pcelld1: usize, // const reg_t PCellD1_R; /* PrimeCell ID 1 */
    pcelld2: usize, // const reg_t PCellD2_R; /* PrimeCell ID 2 */
    pcelld3: usize, // const reg_t PCellD3_R; /* PrimeCell ID 3 */
}

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
            //
            registers.get_mut().lock = registers::GPIO_LOCK_KEY;
            registers.get_mut().cr |= mask;
            registers.get_mut().lock = 0;
        }
        // register_map[port]->DEN_R |= mask;
        registers.get_mut().den |= mask;
        // register_map[port]->DIR_R &= ~mask;
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
        // register_map[port]->DATA[mask] = level ? 0xFF : 0x00;
        match level {
            Level::Low => registers.get_mut().data_mask[mask] = 0,
            Level::High => registers.get_mut().data_mask[mask] = 0xFF,
        }
        // register_map[port]->DIR_R |= mask;
        registers.get_mut().dir |= mask;
        // register_map[port]->DEN_R |= mask;
        registers.get_mut().den |= mask;
    }
}

fn get_port_registers(port: PinPort) -> Unique<Registers> {
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
