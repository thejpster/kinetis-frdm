//! A board support library for the TI Stellaris Launchpad

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

/// Bring in GPIO appropriate for this board
pub use lm4f120h5qr::gpio;

use lm4f120h5qr::pll;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(PartialEq, Clone, Copy)]
pub enum Led {
    Red,
    Blue,
    Green,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Button {
    One,
    Two,
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

pub fn led_on(led: Led) {
    match led {
        Led::Red => gpio::set(LED_RED, gpio::Level::High),
        Led::Blue => gpio::set(LED_BLUE, gpio::Level::High),
        Led::Green => gpio::set(LED_GREEN, gpio::Level::High),
    }
}

pub fn led_off(led: Led) {
    match led {
        Led::Red => gpio::set(LED_RED, gpio::Level::Low),
        Led::Blue => gpio::set(LED_BLUE, gpio::Level::Low),
        Led::Green => gpio::set(LED_GREEN, gpio::Level::Low),
    }
}

pub fn read_button(button: Button) -> gpio::Level {
    match button {
        Button::One => gpio::read(BUTTON_ONE),
        Button::Two => gpio::read(BUTTON_TWO),
    }
}

/// Flash the red LED if we panic
pub fn panic() -> ! {
    loop {
        led_on(Led::Red);
        ::delay(200);
        led_off(Led::Red);
        ::delay(200);
    }
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
