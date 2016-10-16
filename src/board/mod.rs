//! A board support library for the TI Stellaris Launchpad

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

/// Bring in GPIO appropriate for this board
pub use cpu::lm4f120h5qr::gpio;

use cpu::lm4f120h5qr::{fpu, pll, systick};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(PartialEq, Clone, Copy)]
/// The Launchpad has a tri-colour LED, which we consider
/// to be three separate LEDs.
pub enum Led {
    /// The Red LED
    Red,
    /// The Blue LED
    Blue,
    /// The Green LED
    Green,
}

#[derive(PartialEq, Clone, Copy)]
/// The Launchpad has two buttons
pub enum Button {
    /// SW1
    One,
    /// SW2
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

/// The pin used for the Red LED
pub const LED_RED: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin1);
/// The pin used for the Blue LED
pub const LED_BLUE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin2);
/// The pin used for the Green LED
pub const LED_GREEN: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin3);
/// The pin used for Button One
pub const BUTTON_ONE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin0);
/// The pin used for Button Two
pub const BUTTON_TWO: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin4);

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Initialise everything on the board - FPU, PLL, SysTick, GPIO and the LEDs
/// and buttons. Should be pretty much the first call you make in `main()`.
/// Doesn't init the UART - that's separate.
pub fn init() {
    fpu::init();
    pll::init(pll::ClockSpeed::Speed66MHz);
    systick::init();
    gpio::init();
    enable_buttons();
    enable_leds();
}

/// Turn an LED on
pub fn led_on(led: Led) {
    match led {
        Led::Red => gpio::set(LED_RED, gpio::Level::High),
        Led::Blue => gpio::set(LED_BLUE, gpio::Level::High),
        Led::Green => gpio::set(LED_GREEN, gpio::Level::High),
    }
}

/// Turn an LED off
pub fn led_off(led: Led) {
    match led {
        Led::Red => gpio::set(LED_RED, gpio::Level::Low),
        Led::Blue => gpio::set(LED_BLUE, gpio::Level::Low),
        Led::Green => gpio::set(LED_GREEN, gpio::Level::Low),
    }
}

/// Get the state of a button
pub fn read_button(button: Button) -> gpio::Level {
    match button {
        Button::One => gpio::read(BUTTON_ONE),
        Button::Two => gpio::read(BUTTON_TWO),
    }
}

/// Call from a panic handler to flash the red LED quickly.
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
