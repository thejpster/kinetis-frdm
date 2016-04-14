//! A board support library for the TI Stellaris Launchpad

use core::intrinsics::{volatile_store, volatile_load};
use gpio;
use lm4f120h5qr;

pub const LED_RED: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin1);
pub const LED_BLUE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin2);
pub const LED_GREEN: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin3);
pub const BUTTON_ONE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin0);
pub const BUTTON_TWO: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin4);

pub fn init() {
    pll_init();
    gpio::init();
    enable_buttons();
    enable_leds();
    // gpio_enable_uart(UART_ID_0);
}

fn enable_buttons() {
    gpio::set_direction(BUTTON_ONE, gpio::PinMode::InputPull(gpio::Level::High));
    gpio::set_direction(BUTTON_TWO, gpio::PinMode::InputPull(gpio::Level::High));
}

fn enable_leds() {
    gpio::set_direction(LED_RED, gpio::PinMode::Output);
    gpio::set_direction(LED_BLUE, gpio::PinMode::Output);
    gpio::set_direction(LED_GREEN, gpio::PinMode::Output);
}

fn pll_init() {
    unsafe {
        let mut rcc: usize = volatile_load(lm4f120h5qr::SYSCTL_RCC_R);
        // RCC SYSDIV field = 0x0
        rcc &= !lm4f120h5qr::SYSCTL_RCC_SYSDIV_M;
        // XTAL field = 0x15 (=> 16MHz)
        rcc &= !lm4f120h5qr::SYSCTL_RCC_XTAL_M;
        rcc |= lm4f120h5qr::SYSCTL_RCC_XTAL_16MHZ;
        // Set BYPASS bit
        rcc |= lm4f120h5qr::SYSCTL_RCC_BYPASS;
        // Set to MOSC
        rcc &= !lm4f120h5qr::SYSCTL_RCC_OSCSRC_M;
        rcc |= lm4f120h5qr::SYSCTL_RCC_OSCSRC_MAIN;
        // Disable PIOSC
        rcc |= lm4f120h5qr::SYSCTL_RCC_IOSCDIS;
        // Enable MOSC (i.e. don't disable)
        rcc &= !lm4f120h5qr::SYSCTL_RCC_MOSCDIS;
        // Write to register
        volatile_store(lm4f120h5qr::SYSCTL_RCC_R, rcc);
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
