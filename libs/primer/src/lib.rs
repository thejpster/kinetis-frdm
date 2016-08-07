//! Primer is a library for simple bare-metal ARM programming.

#![feature(allocator)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(start)]
#![feature(unique)]
#![allocator]
#![no_std]
#![warn(dead_code)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod board;
pub mod lm4f120h5qr;
pub mod common;

extern crate spin;
extern crate rlibc;
extern crate linked_list_allocator;
#[macro_use] extern crate lazy_static;

pub use lm4f120h5qr::delay;

extern "C" {
    static mut _start_data_flash: usize;
    static mut _start_data: usize;
    static mut _end_data: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
    fn _stack_top();
    fn primer_start();
}

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

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
    loop {}
}

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> ! {
    loop {}
}

/// Required by modules that haven't been build with panic = "abort"
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

/// Required by the compiler.
#[lang="eh_personality"]
extern "C" fn eh_personality() {}

/// Entry point of panic from the libcore crate.
///
/// Required by the compiler.
#[lang="panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

/// A Non Maskable Interrupt (NMI) can be signalled by a peripheral or
/// triggered by software. This is the highest priority exception other than
/// reset. It is permanently enabled and has a fixed priority of -2. NMIs
/// cannot be:
/// * masked or prevented from activation by any other exception
/// * preempted by any exception other than Reset.
#[no_mangle]
pub unsafe extern "C" fn isr_nmi() {
    loop {}
}

/// A HardFault is an exception that occurs because of an error during
/// exception processing, or because an exception cannot be managed by any
/// other exception mechanism. HardFaults have a fixed priority of -1, meaning
/// they have higher priority than any exception with configurable priority.
#[no_mangle]
pub unsafe extern "C" fn isr_hardfault() {
    loop {}
}

/// A MemManage fault is an exception that occurs because of a memory
/// protection related fault. The the fixed memory protection constraints
/// determines this fault, for both instruction and data memory transactions.
/// This fault is always used to abort instruction accesses to Execute Never
/// (XN) memory regions.
#[no_mangle]
pub unsafe extern "C" fn isr_mmfault() {
    loop {}
}

/// A BusFault is an exception that occurs because of a memory related fault
/// for an instruction or data memory transaction. This might be from an error
/// detected on a bus in the memory system.
#[no_mangle]
pub unsafe extern "C" fn isr_busfault() {
    loop {}
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
    loop {}
}

/// A supervisor call (SVC) is an exception that is triggered by the SVC
/// instruction. In an OS environment, applications can use SVC instructions
/// to access OS kernel functions and device drivers.
#[no_mangle]
pub unsafe extern "C" fn isr_svcall() {
    loop {}
}

/// Debug monitor interrupt handler.
#[no_mangle]
pub unsafe extern "C" fn isr_debugmon() {
    loop {}
}

/// PendSV is an interrupt-driven request for system-level service. In an OS
/// environment, use PendSV for context switching when no other exception is
/// active.
#[no_mangle]
pub unsafe extern "C" fn isr_pendsv() {
    loop {}
}

/// A SysTick exception is an exception the system timer generates when it
/// reaches zero. Software can also generate a SysTick exception. In an OS
/// environment, the processor can use this exception as system tick.
#[no_mangle]
pub unsafe extern "C" fn isr_systick() {
    loop {}
}

/// A place-holder ISR used when we have nothing better to use.
#[no_mangle]
pub unsafe extern "C" fn isr_empty_def() {
    loop {}
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
