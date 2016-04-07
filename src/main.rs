#![feature(lang_items,core_intrinsics,asm,start)]
#![no_std]
#![crate_type="staticlib"]

extern crate blink;

use core::intrinsics::{volatile_store, volatile_load};

use blink::lm4f120h5qr;
use blink::gpio;
use blink::launchpad;

// #[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"]
extern "C" fn eh_personality() {}
#[lang="panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}

extern "C" {
    static mut _start_data_flash: usize;
    static mut _start_data: usize;
    static mut _end_data: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
    fn _stack_top();
}

#[link_section=".nvic_table"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 155] = [
    // Stack pointer
    Some(_stack_top),
    // Reset
    Some(startup),
    // NMI
    Some(isr_nmi),
    // Hard Fault
    Some(isr_hardfault),
    // CM3 Memory Management Fault
    Some(isr_mmfault),
    // CM3 Bus Fault
    Some(isr_busfault),
    // CM3 Usage Fault
    Some(isr_usagefault),
    // Reserved - Used as NXP Checksum
    None,
    // Reserved
    None,
    // Reserved
    None,
    // Reserved
    None,
    // SVCall
    Some(isr_svcall),
    // Reserved for debug
    Some(isr_debugmon),
    // Reserved
    None,
    // PendSV
    Some(isr_pendsv),
    // SysTick
    Some(isr_systick),
    // GPIO Port A                      16
    Some(isr_empty_def),
    // GPIO Port B                      17
    Some(isr_empty_def),
    // GPIO Port C                      18
    Some(isr_empty_def),
    // GPIO Port D                      19
    Some(isr_empty_def),
    // GPIO Port E                      20
    Some(isr_empty_def),
    // UART 0                           21
    Some(isr_empty_def),
    // UART 1                           22
    Some(isr_empty_def),
    // SSI 0                            23
    Some(isr_empty_def),
    // I2C 0                            24
    Some(isr_empty_def),
    // Reserved                         25
    None,
    // Reserved                         26
    None,
    // Reserved                         27
    None,
    // Reserved                         28
    None,
    // Reserved                         29
    None,
    // ADC 0 Seq 0                      30
    Some(isr_empty_def),
    // ADC 0 Seq 1                      31
    Some(isr_empty_def),
    // ADC 0 Seq 2                      32
    Some(isr_empty_def),
    // ADC 0 Seq 3                      33
    Some(isr_empty_def),
    // WDT 0 and 1                      34
    Some(isr_empty_def),
    // 16/32 bit timer 0 A              35
    Some(isr_empty_def),
    // 16/32 bit timer 0 B              36
    Some(isr_empty_def),
    // 16/32 bit timer 1 A              37
    Some(isr_empty_def),
    // 16/32 bit timer 1 B              38
    Some(isr_empty_def),
    // 16/32 bit timer 2 A              39
    Some(isr_empty_def),
    // 16/32 bit timer 2 B              40
    Some(isr_empty_def),
    // Analog comparator 0              41
    Some(isr_empty_def),
    // Analog comparator 1              42
    Some(isr_empty_def),
    // Reserved                         43
    None,
    // System control                   44
    Some(isr_empty_def),
    // Flash + EEPROM control           45
    Some(isr_empty_def),
    // GPIO Port F                      46
    Some(isr_empty_def),
    // Reserved                         47
    None,
    // Reserved                         48
    None,
    // UART 2                           49
    Some(isr_empty_def),
    // SSI 1                            50
    Some(isr_empty_def),
    // 16/32 bit timer 3 A              51
    Some(isr_empty_def),
    // 16/32 bit timer 3 B              52
    Some(isr_empty_def),
    // I2C 1                            53
    Some(isr_empty_def),
    // Reserved                         54
    None,
    // CAN 0                            55
    Some(isr_empty_def),
    // Reserved                         56
    None,
    // Reserved                         57
    None,
    // Reserved                         58
    None,
    // Hibernation module               59
    Some(isr_empty_def),
    // USB                              60
    Some(isr_empty_def),
    // Reserved                         61
    None,
    // UDMA SW                          62
    Some(isr_empty_def),
    // UDMA Error                       63
    Some(isr_empty_def),
    // ADC 1 Seq 0                      64
    Some(isr_empty_def),
    // ADC 1 Seq 1                      65
    Some(isr_empty_def),
    // ADC 1 Seq 2                      66
    Some(isr_empty_def),
    // ADC 1 Seq 3                      67
    Some(isr_empty_def),
    // Reserved                         68
    None,
    // Reserved                         69
    None,
    // Reserved                         70
    None,
    // Reserved                         71
    None,
    // Reserved                         72
    None,
    // SSI 2                            73
    Some(isr_empty_def),
    // SSI 2                            74
    Some(isr_empty_def),
    // UART 3                           75
    Some(isr_empty_def),
    // UART 4                           76
    Some(isr_empty_def),
    // UART 5                           77
    Some(isr_empty_def),
    // UART 6                           78
    Some(isr_empty_def),
    // UART 7                           79
    Some(isr_empty_def),
    // Reserved                         80
    None,
    // Reserved                         81
    None,
    // Reserved                         82
    None,
    // Reserved                         83
    None,
    // I2C 2                            84
    Some(isr_empty_def),
    // I2C 4                            85
    Some(isr_empty_def),
    // 16/32 bit timer 4 A              86
    Some(isr_empty_def),
    // 16/32 bit timer 4 B              87
    Some(isr_empty_def),
    // Reserved                         88
    None,
    // Reserved                         89
    None,
    // Reserved                         90
    None,
    // Reserved                         91
    None,
    // Reserved                         92
    None,
    // Reserved                         93
    None,
    // Reserved                         94
    None,
    // Reserved                         95
    None,
    // Reserved                         96
    None,
    // Reserved                         97
    None,
    // Reserved                         98
    None,
    // Reserved                         99
    None,
    // Reserved                         100
    None,
    // Reserved                         101
    None,
    // Reserved                         102
    None,
    // Reserved                         103
    None,
    // Reserved                         104
    None,
    // Reserved                         105
    None,
    // Reserved                         106
    None,
    // Reserved                         107
    None,
    // 16/32 bit timer 5 A              108
    Some(isr_empty_def),
    // 16/32 bit timer 5 B              109
    Some(isr_empty_def),
    // 32/64 bit timer 0 A              110
    Some(isr_empty_def),
    // 32/64 bit timer 0 B              111
    Some(isr_empty_def),
    // 32/64 bit timer 1 A              112
    Some(isr_empty_def),
    // 32/64 bit timer 1 B              113
    Some(isr_empty_def),
    // 32/64 bit timer 2 A              114
    Some(isr_empty_def),
    // 32/64 bit timer 2 B              115
    Some(isr_empty_def),
    // 32/64 bit timer 3 A              116
    Some(isr_empty_def),
    // 32/64 bit timer 3 B              117
    Some(isr_empty_def),
    // 32/64 bit timer 4 A              118
    Some(isr_empty_def),
    // 32/64 bit timer 4 B              119
    Some(isr_empty_def),
    // 32/64 bit timer 5 A              120
    Some(isr_empty_def),
    // 32/64 bit timer 5 B              121
    Some(isr_empty_def),
    // System Exception                 122
    Some(isr_empty_def),
    // Reserved                         123
    None,
    // Reserved                         124
    None,
    // Reserved                         125
    None,
    // Reserved                         126
    None,
    // Reserved                         127
    None,
    // Reserved                         128
    None,
    // Reserved                         129
    None,
    // Reserved                         130
    None,
    // Reserved                         131
    None,
    // Reserved                         132
    None,
    // Reserved                         133
    None,
    // Reserved                         134
    None,
    // Reserved                         135
    None,
    // Reserved                         136
    None,
    // Reserved                         137
    None,
    // Reserved                         138
    None,
    // Reserved                         139
    None,
    // Reserved                         140
    None,
    // Reserved                         141
    None,
    // Reserved                         142
    None,
    // Reserved                         143
    None,
    // Reserved                         144
    None,
    // Reserved                         145
    None,
    // Reserved                         146
    None,
    // Reserved                         147
    None,
    // Reserved                         148
    None,
    // Reserved                         149
    None,
    // Reserved                         150
    None,
    // Reserved                         151
    None,
    // Reserved                         152
    None,
    // Reserved                         153
    None,
    // Reserved                         154
    None
];

pub unsafe extern "C" fn startup() {
    let mut src: *mut usize = &mut _start_data_flash;
    let mut dest: *mut usize = &mut _start_data;

    while dest < &mut _end_data as *mut usize {
        *dest = *src;
        dest = ((dest as usize) + 4) as *mut usize;
        src = ((src as usize) + 4) as *mut usize;
    }

    dest = &mut _bss_start as *mut usize;

    while dest < &mut _end_data as *mut usize {
        *dest = 0;
        dest = ((dest as usize) + 4) as *mut usize;
    }

    // Set up clock
    {
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

    launchpad::init();

    rust_loop();
}

pub fn led_on() {
    gpio::set(launchpad::LED_RED, gpio::Level::High);
}

pub fn led_off() {
    gpio::set(launchpad::LED_RED, gpio::Level::Low);
}

pub fn delay(ms: i32) {
    for _ in 0..ms * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}

pub fn rust_loop() {
    loop {
        led_on();
        delay(1000);
        led_off();
        delay(1000);
    }
}

#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        startup();
    }
    0
}


pub unsafe extern "C" fn isr_nmi() {
    loop {}
}
pub unsafe extern "C" fn isr_hardfault() {
    loop {}
}
pub unsafe extern "C" fn isr_mmfault() {
    loop {}
}
pub unsafe extern "C" fn isr_busfault() {
    loop {}
}
pub unsafe extern "C" fn isr_usagefault() {
    loop {}
}
pub unsafe extern "C" fn isr_svcall() {
    loop {}
}
pub unsafe extern "C" fn isr_debugmon() {
    loop {}
}
pub unsafe extern "C" fn isr_pendsv() {
    loop {}
}
pub unsafe extern "C" fn isr_systick() {
    loop {}
}
pub unsafe extern "C" fn isr_empty_def() {
    loop {}
}
