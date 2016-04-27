//! A board support library for the TI Stellaris Launchpad

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub use lm4f120h5qr::gpio;
use lm4f120h5qr::pll;

// ****************************************************************************
//
// Public Types
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
// Public Data
//
// ****************************************************************************

pub const LED_RED: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin1);
pub const LED_BLUE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin2);
pub const LED_GREEN: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin3);
pub const BUTTON_ONE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin0);
pub const BUTTON_TWO: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin4);

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

pub fn init() {
    pll::init(pll::ClockSpeed::Speed66MHz);
    gpio::init();
    enable_buttons();
    enable_leds();
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn enable_buttons() {
    gpio::set_direction(BUTTON_ONE, gpio::PinMode::InputPull(gpio::Level::High));
    gpio::set_direction(BUTTON_TWO, gpio::PinMode::InputPull(gpio::Level::High));
}

fn enable_leds() {
    gpio::set_direction(LED_RED, gpio::PinMode::Output);
    gpio::set_direction(LED_BLUE, gpio::PinMode::Output);
    gpio::set_direction(LED_GREEN, gpio::PinMode::Output);
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
