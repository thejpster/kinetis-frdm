//! Modules specific to the TI LM4F120H5QR Cortex-M4 microcontroller

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod gpio;
pub mod pll;
pub mod registers;
pub mod uart;
pub mod timer;

extern "C" {
    fn _stack_top();
}

use ::common::startup;

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

#[link_section=".nvic_table"]
#[no_mangle]
pub static ISR_VECTORS: [Option<unsafe extern "C" fn()>; 155] = [// Stack pointer
                                                                 Some(_stack_top),
                                                                 // Reset
                                                                 Some(startup::startup),
                                                                 // NMI
                                                                 Some(startup::isr_nmi),
                                                                 // Hard Fault
                                                                 Some(startup::isr_hardfault),
                                                                 // CM3 Memory Management Fault
                                                                 Some(startup::isr_mmfault),
                                                                 // CM3 Bus Fault
                                                                 Some(startup::isr_busfault),
                                                                 // CM3 Usage Fault
                                                                 Some(startup::isr_usagefault),
                                                                 // Reserved - Used as NXP Checksum
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // SVCall
                                                                 Some(startup::isr_svcall),
                                                                 // Reserved for debug
                                                                 Some(startup::isr_debugmon),
                                                                 // Reserved
                                                                 None,
                                                                 // PendSV
                                                                 Some(startup::isr_pendsv),
                                                                 // SysTick
                                                                 Some(startup::isr_systick),
                                                                 // GPIO Port A                      16
                                                                 Some(startup::isr_empty_def),
                                                                 // GPIO Port B                      17
                                                                 Some(startup::isr_empty_def),
                                                                 // GPIO Port C                      18
                                                                 Some(startup::isr_empty_def),
                                                                 // GPIO Port D                      19
                                                                 Some(startup::isr_empty_def),
                                                                 // GPIO Port E                      20
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 0                           21
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 1                           22
                                                                 Some(startup::isr_empty_def),
                                                                 // SSI 0                            23
                                                                 Some(startup::isr_empty_def),
                                                                 // I2C 0                            24
                                                                 Some(startup::isr_empty_def),
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
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 0 Seq 1                      31
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 0 Seq 2                      32
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 0 Seq 3                      33
                                                                 Some(startup::isr_empty_def),
                                                                 // WDT 0 and 1                      34
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 0 A              35
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 0 B              36
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 1 A              37
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 1 B              38
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 2 A              39
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 2 B              40
                                                                 Some(startup::isr_empty_def),
                                                                 // Analog comparator 0              41
                                                                 Some(startup::isr_empty_def),
                                                                 // Analog comparator 1              42
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         43
                                                                 None,
                                                                 // System control                   44
                                                                 Some(startup::isr_empty_def),
                                                                 // Flash + EEPROM control           45
                                                                 Some(startup::isr_empty_def),
                                                                 // GPIO Port F                      46
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         47
                                                                 None,
                                                                 // Reserved                         48
                                                                 None,
                                                                 // UART 2                           49
                                                                 Some(startup::isr_empty_def),
                                                                 // SSI 1                            50
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 3 A              51
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 3 B              52
                                                                 Some(startup::isr_empty_def),
                                                                 // I2C 1                            53
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         54
                                                                 None,
                                                                 // CAN 0                            55
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         56
                                                                 None,
                                                                 // Reserved                         57
                                                                 None,
                                                                 // Reserved                         58
                                                                 None,
                                                                 // Hibernation module               59
                                                                 Some(startup::isr_empty_def),
                                                                 // USB                              60
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         61
                                                                 None,
                                                                 // UDMA SW                          62
                                                                 Some(startup::isr_empty_def),
                                                                 // UDMA Error                       63
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 1 Seq 0                      64
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 1 Seq 1                      65
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 1 Seq 2                      66
                                                                 Some(startup::isr_empty_def),
                                                                 // ADC 1 Seq 3                      67
                                                                 Some(startup::isr_empty_def),
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
                                                                 Some(startup::isr_empty_def),
                                                                 // SSI 2                            74
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 3                           75
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 4                           76
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 5                           77
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 6                           78
                                                                 Some(startup::isr_empty_def),
                                                                 // UART 7                           79
                                                                 Some(startup::isr_empty_def),
                                                                 // Reserved                         80
                                                                 None,
                                                                 // Reserved                         81
                                                                 None,
                                                                 // Reserved                         82
                                                                 None,
                                                                 // Reserved                         83
                                                                 None,
                                                                 // I2C 2                            84
                                                                 Some(startup::isr_empty_def),
                                                                 // I2C 4                            85
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 4 A              86
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 4 B              87
                                                                 Some(startup::isr_empty_def),
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
                                                                 Some(startup::isr_empty_def),
                                                                 // 16/32 bit timer 5 B              109
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 0 A              110
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 0 B              111
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 1 A              112
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 1 B              113
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 2 A              114
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 2 B              115
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 3 A              116
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 3 B              117
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 4 A              118
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 4 B              119
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 5 A              120
                                                                 Some(startup::isr_empty_def),
                                                                 // 32/64 bit timer 5 B              121
                                                                 Some(startup::isr_empty_def),
                                                                 // System Exception                 122
                                                                 Some(startup::isr_empty_def),
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
                                                                 None];

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Busy-waits for the given period.
///
/// Currently this function spins with a empirical number
/// of NOPS per millisecond. It should really use a timer.
///
/// * `ms` - The period to wait, in milliseconds
pub fn delay(ms: i32) {
    for _ in 0..ms * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
