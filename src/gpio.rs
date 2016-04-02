
use core::intrinsics::{volatile_store, volatile_load};
use lm4f120h5qr::*;

struct Registers
{
    data_arr: [u32; 255], //reg_t DATA[255]; /* Data - offset sets pin mask */
    data: u32, //reg_t DATA_R; /* Data register - sets all pins */
    dir: u32, //reg_t DIR_R; /* Direction */
    is: u32, //reg_t IS_R; /* Interrupt Sense */
    ibe: u32, //reg_t IBE_R; /* Interrupt Both Edges */
    iev: u32, //reg_t IEV_R; /* Interrupt Event */
    im: u32, //reg_t IM_R; /* Interrupt Mask */
    ris: u32, //const reg_t RIS_R; /* Raw Interrupt Status */
    mis: u32, //const reg_t MIS_R; /* Masked Interrupt Status */
    icr: u32, //reg_t ICR_R; /* Interrupt Clear */
    afsel: u32, //reg_t AFSEL_R; /* Alternate function Select */
    _padding: [u32; 55], //const reg_t _padding[55];
    dr2r: u32, //reg_t DR2R_R; /* 2mA drive select */
    dr4r: u32, //reg_t DR4R_R; /* 4mA drive select */
    dr8r: u32, //reg_t DR8R_R; /* 8mA drive select */
    odr: u32, //reg_t ODR_R; /* Open-drain select */
    pur: u32, //reg_t PUR_R; /* Pull-up select */
    pdr: u32, //reg_t PDR_R; /* Pull-down select */
    slr: u32, //reg_t SLR_R; /* Slew-rate control */
    den: u32, //reg_t DEN_R; /* Digital enable */
    lock: u32, //reg_t LOCK_R; /* Lock */
    cr: u32, //reg_t CR_R; /* Commit */
    amsel: u32, //reg_t AMSEL_R; /* Analog mode select */
    pctl: u32, //reg_t PCTL_R; /* Port Control */
    adcctl: u32, //reg_t ADCCTL_R; /* ADC Control */
    dmactl: u32, //reg_t DMACTL_R; /* DMA Control */
    _padding2: [u32; 678], //const reg_t _padding2[678];
    periphid4: u32, //const reg_t PeriphID4_R; /* Peripheral ID 4 */
    periphid5: u32, //const reg_t PeriphID5_R; /* Peripheral ID 5 */
    periphid6: u32, //const reg_t PeriphID6_R; /* Peripheral ID 6 */
    periphid7: u32, //const reg_t PeriphID7_R; /* Peripheral ID 7 */
    periphid0: u32, //const reg_t PeriphID0_R; /* Peripheral ID 0 */
    periphid1: u32, //const reg_t PeriphID1_R; /* Peripheral ID 1 */
    periphid2: u32, //const reg_t PeriphID2_R; /* Peripheral ID 2 */
    periphid3: u32, //const reg_t PeriphID3_R; /* Peripheral ID 3 */
    pcelld0: u32, //const reg_t PCellD0_R; /* PrimeCell ID 0 */
    pcelld1: u32, //const reg_t PCellD1_R; /* PrimeCell ID 1 */
    pcelld2: u32, //const reg_t PCellD2_R; /* PrimeCell ID 2 */
    pcelld3: u32, //const reg_t PCellD3_R; /* PrimeCell ID 3 */
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum PinPort {
    PortA(Pin),
    PortB(Pin),
    PortC(Pin),
    PortD(Pin),
    PortE(Pin),
    PortF(Pin)
}

pub enum PinMode {
    InputPull(Level),
    Input,
    Output
}

pub enum Level {
    High,
    Low
}

pub fn init() {
    unimplemented!();
}

pub fn set_direction(pinport: PinPort, mode: PinMode) {
    match mode {
        PinMode::InputPull(Level::High) => make_input_pullup(&pinport),
        PinMode::InputPull(Level::Low) => make_input_pulldown(&pinport),
        PinMode::Input => make_input(&pinport),
        PinMode::Output => make_output(&pinport)
    }
}

pub fn set(pinport: PinPort, level: Level) {
    unimplemented!();
}

pub fn read(pinport: PinPort) -> Level {
    unimplemented!();
}

fn get_port_mask(port: &PinPort) -> u32 {
    match *port {
        PinPort::PortA(_) => 1<<0,
        PinPort::PortB(_) => 1<<1,
        PinPort::PortC(_) => 1<<2,
        PinPort::PortD(_) => 1<<3,
        PinPort::PortE(_) => 1<<4,
        PinPort::PortF(_) => 1<<5,
    }
}

fn get_pin_mask(pinport: &PinPort) -> u32 {
    let pin = match *pinport {
        PinPort::PortA(ref x) => x,
        PinPort::PortB(ref x) => x,
        PinPort::PortC(ref x) => x,
        PinPort::PortD(ref x) => x,
        PinPort::PortE(ref x) => x,
        PinPort::PortF(ref x) => x,
    };
    match *pin {
        Pin::Pin0 => 1<<0 as u32,
        Pin::Pin1 => 1<<1 as u32,
        Pin::Pin2 => 1<<2 as u32,
        Pin::Pin3 => 1<<3 as u32,
        Pin::Pin4 => 1<<4 as u32,
        Pin::Pin5 => 1<<5 as u32,
        Pin::Pin6 => 1<<6 as u32,
        Pin::Pin7 => 1<<7 as u32,
    }
}

fn enable_port(port: &PinPort) {
    let mask = get_port_mask(port);
    unsafe {
        let mut reg:u32 = volatile_load(SYSCTL_RCGCGPIO_R);
        if (reg & mask) == 0 {
            reg |= mask;
            volatile_store(SYSCTL_RCGCGPIO_R, reg);
            // Wait for module to settle
            while (volatile_load(SYSCTL_RCGCGPIO_R) & mask) == 0 {
                asm!("NOP");
            }
        }
    }
}

fn force_gpio_periph(pinport: &PinPort) {

}

fn make_input(pinport: &PinPort) {
    enable_port(pinport);
    if *pinport == PinPort::PortF(Pin::Pin0) {
        /* The GPIO for button one is multiplexed with NMI so we
         * have to 'unlock' it before we can use it
         */
        //register_map[GPIO_PORT_F]->LOCK_R = GPIO_LOCK_KEY; /* Unlock CR  */
        //register_map[GPIO_PORT_F]->CR_R |= GPIO_GET_PIN(pin); /* Allow F0 to be changed */
        //register_map[GPIO_PORT_F]->LOCK_R = 0; /* Lock CR again */
    }
    //register_map[port]->DEN_R |= mask;
    //register_map[port]->DIR_R &= ~mask;
    force_gpio_periph(pinport);

}

fn make_input_pullup(pinport: &PinPort) {
    make_input(pinport);
}

fn make_input_pulldown(pinport: &PinPort) {
    make_input(pinport);
}

fn make_output(pinport: &PinPort) {
    enable_port(pinport);
}

