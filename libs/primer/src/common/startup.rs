//! Handles CPU startup (segment init, etc) that's common to all CPUs.
//! Chip specific init (like the interrupt table) should be in the relevant <chip> module.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern "C" {
    static mut _start_data_flash: usize;
    static mut _start_data: usize;
    static mut _end_data: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
    fn primer_start();
}

use ::board::launchpad;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
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
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Performs what you might otherwise call 'C Startup'
///
/// Copies global .data init from flash to SRAM and then
/// zeros the bss segment.
#[no_mangle]
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

    primer_start();
}

/// A Non Maskable Interrupt (NMI) can be signalled by a peripheral or
/// triggered by software. This is the highest priority exception other than
/// reset. It is permanently enabled and has a fixed priority of -2. NMIs
/// cannot be:
/// * masked or prevented from activation by any other exception
/// * preempted by any exception other than Reset.
#[no_mangle]
pub unsafe extern "C" fn isr_nmi() {
    // Do nothing
}

/// A HardFault is an exception that occurs because of an error during
/// exception processing, or because an exception cannot be managed by any
/// other exception mechanism. HardFaults have a fixed priority of -1, meaning
/// they have higher priority than any exception with configurable priority.
#[no_mangle]
pub unsafe extern "C" fn isr_hardfault() {
    loop {
        launchpad::led_on(launchpad::Led::Green);
        ::delay(200);
        launchpad::led_off(launchpad::Led::Green);
        ::delay(200);
    }
}

/// A MemManage fault is an exception that occurs because of a memory
/// protection related fault. The the fixed memory protection constraints
/// determines this fault, for both instruction and data memory transactions.
/// This fault is always used to abort instruction accesses to Execute Never
/// (XN) memory regions.
#[no_mangle]
pub unsafe extern "C" fn isr_mmfault() {
    loop {
        launchpad::led_on(launchpad::Led::Green);
        ::delay(200);
        launchpad::led_off(launchpad::Led::Green);
        ::delay(200);
    }
}

/// A BusFault is an exception that occurs because of a memory related fault
/// for an instruction or data memory transaction. This might be from an error
/// detected on a bus in the memory system.
#[no_mangle]
pub unsafe extern "C" fn isr_busfault() {
    loop {
        launchpad::led_on(launchpad::Led::Green);
        ::delay(200);
        launchpad::led_off(launchpad::Led::Green);
        ::delay(200);
    }
}

/// A UsageFault is an exception that occurs because of a fault related to instruction execution. This includes:
/// * an undefined instruction
/// * an illegal unaligned access
/// * invalid state on instruction execution
/// * an error on exception return.
/// The following can cause a UsageFault when the core is configured to report them:
/// * an unaligned address on word and halfword memory access
/// * division by zero.
#[no_mangle]
pub unsafe extern "C" fn isr_usagefault() {
    loop {
        launchpad::led_on(launchpad::Led::Green);
        ::delay(200);
        launchpad::led_off(launchpad::Led::Green);
        ::delay(200);
    }
}

/// A supervisor call (SVC) is an exception that is triggered by the SVC
/// instruction. In an OS environment, applications can use SVC instructions
/// to access OS kernel functions and device drivers.
#[no_mangle]
pub unsafe extern "C" fn isr_svcall() {
    // Nothing
}

/// Debug monitor interrupt handler.
#[no_mangle]
pub unsafe extern "C" fn isr_debugmon() {
    // Nothing
}

/// PendSV is an interrupt-driven request for system-level service. In an OS
/// environment, use PendSV for context switching when no other exception is
/// active.
#[no_mangle]
pub unsafe extern "C" fn isr_pendsv() {
    // Nothing
}

/// A SysTick exception is an exception the system timer generates when it
/// reaches zero. Software can also generate a SysTick exception. In an OS
/// environment, the processor can use this exception as system tick.
#[no_mangle]
pub unsafe extern "C" fn isr_systick() {
    // Nothing
}

/// A place-holder ISR used when we have nothing better to use.
#[no_mangle]
pub unsafe extern "C" fn isr_empty_def() {
    loop {
        launchpad::led_on(launchpad::Led::Green);
        ::delay(200);
        launchpad::led_off(launchpad::Led::Green);
        ::delay(200);
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
