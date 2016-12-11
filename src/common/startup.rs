//! Handles CPU startup (segment init, etc) that's common to all CPUs.
//! Chip specific init (like the interrupt table) should be in the relevant <chip> module.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use r0;
use cortex_m;
use alloc_cortex_m;

use board;
use cpu::{systick, uart};

extern "C" {
    // These symbols come from the linker script
    static mut _data_start_flash: usize;
    static mut _data_start: usize;
    static mut _data_end: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
    static mut _heap_start: usize;
    static mut _heap_end: usize;
    // This is defined by your application
    fn main();
    fn _stack_top();
}


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

/// Contains exception handlers, interrupt service routine vectors and a magic value
/// that marks the start of the stack.
#[link_section=".nvic_table"]
#[no_mangle]
pub static ISR_VECTORS: [Option<cortex_m::Handler>; 48] = [// Stack pointer
                                                           Some(_stack_top),
                                                           // Reset
                                                           Some(reset_vector),
                                                           // NMI
                                                           Some(isr_nmi),
                                                           // Hard Fault
                                                           Some(isr_hardfault),
                                                           // Unused
                                                           None,
                                                           // CM0+ Bus Fault
                                                           Some(isr_busfault),
                                                           // CM0+ Usage Fault
                                                           Some(isr_usagefault),
                                                           // Unused
                                                           None,
                                                           // Unused
                                                           None,
                                                           // Unused
                                                           None,
                                                           // Unused
                                                           None,
                                                           // Supervisor call
                                                           Some(isr_svcall),
                                                           // Debug monitor
                                                           Some(isr_debugmon),
                                                           // Unused
                                                           None,
                                                           // Pendable service request
                                                           Some(isr_pendsv),
                                                           // SysTick
                                                           Some(isr_systick),
                                                           // DMA Channel 0 transfer complete
                                                           None,
                                                           // DMA Channel 0 transfer complete
                                                           None,
                                                           // DMA Channel 0 transfer complete
                                                           None,
                                                           // DMA Channel 0 transfer complete
                                                           None,
                                                           // MCM
                                                           None,
                                                           // Flash
                                                           None,
                                                           // PMC
                                                           None,
                                                           // LLWU
                                                           None,
                                                           // I2C0
                                                           None,
                                                           // Reserved
                                                           None,
                                                           // SPI0
                                                           None,
                                                           // SPI1
                                                           None,
                                                           // SCI0
                                                           None,
                                                           // SCI1
                                                           None,
                                                           // SCI2
                                                           None,
                                                           // ADC0
                                                           None,
                                                           // ACMP0
                                                           None,
                                                           // FTM0
                                                           None,
                                                           // FTM1
                                                           None,
                                                           // FTM2
                                                           None,
                                                           // RTC
                                                           None,
                                                           // ACMP1
                                                           None,
                                                           // PIT_CH0
                                                           None,
                                                           // PIT_CH1
                                                           None,
                                                           // KBI0
                                                           None,
                                                           // KBI1
                                                           None,
                                                           // Reserved
                                                           None,
                                                           // ICS
                                                           None,
                                                           // Watchdog
                                                           None,
                                                           // Reserved
                                                           None,
                                                           // Reserved
                                                           None,
                                                           // Reserved
                                                           None];

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

/// Kind of exception
#[derive(Debug)]
enum Exception {
    /// i.e. currently not servicing an exception
    ThreadMode,
    /// Non-maskable interrupt.
    Nmi,
    /// All class of fault.
    HardFault,
    /// Memory management.
    MemoryManagementFault,
    /// Pre-fetch fault, memory access fault.
    BusFault,
    /// Undefined instruction or illegal state.
    UsageFault,
    /// System service call via SWI instruction
    SVCall,
    /// Pendable request for system service
    PendSV,
    /// System tick timer
    Systick,
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)]
    Reserved,
}

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

/// Performs what you might otherwise call 'C Startup'.
/// This routine is specified at the reset vector in the ISR vector table.
///
/// Copies global .data init from flash to SRAM and then
/// zeros the bss segment.
pub unsafe extern "C" fn reset_vector() {
    let data_start_flash: *mut usize = &mut _data_start_flash;
    let data_start: *mut usize = &mut _data_start;
    let data_end: *mut usize = &mut _data_end;
    let bss_start: *mut usize = &mut _bss_start;
    let bss_end: *mut usize = &mut _bss_end;
    let heap_start: *mut usize = &mut _heap_start;
    let heap_end: *mut usize = &mut _heap_end;

    r0::init_data(data_start, data_end, data_start_flash);
    r0::zero_bss(bss_start, bss_end);

    alloc_cortex_m::init(heap_start, heap_end);

    board::init();
    main();
    loop {}
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

impl Exception {
    /// Returns the exception that's currently being serviced
    pub fn current() -> Exception {
        match cortex_m::peripheral::scb().icsr.read() as u8 {
            0 => Exception::ThreadMode,
            2 => Exception::Nmi,
            3 => Exception::HardFault,
            4 => Exception::MemoryManagementFault,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            11 => Exception::SVCall,
            14 => Exception::PendSV,
            15 => Exception::Systick,
            n if n >= 16 => Exception::Interrupt(n - 16),
            _ => Exception::Reserved,
        }
    }
}

/// A HardFault is an exception that occurs because of an error during
/// exception processing, or because an exception cannot be managed by any
/// other exception mechanism. HardFaults have a fixed priority of -1, meaning
/// they have higher priority than any exception with configurable priority.
///
/// This function came from japaric's excellent F3 crate.
#[naked]
pub unsafe extern "C" fn isr_hardfault() {
    use core::intrinsics;

    // NOTE need asm!, #[naked] and unreachable() to avoid modifying the stack
    // pointer (MSP)
    asm!("mrs r0, MSP
          ldr r1, [r0, #20]
          bl isr_hardfault_rs" :::: "volatile");

    intrinsics::unreachable();
}

/// Once the StacFrame has been gathered by `isr_hardfault`, we can process the
/// HardFault using a normal Rust function. It's `no_mangle` because we
/// refer to it from raw assembler in `isr_hardfault`.
#[no_mangle]
pub unsafe extern "C" fn isr_hardfault_rs(_sf: &cortex_m::StackFrame) -> ! {
    // Need ITM support for this to work
    // iprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), sf.pc);

    // We can see this in the debugger
    let _exc = Exception::current();

    bkpt!();

    loop {}
}

/// A Non Maskable Interrupt (NMI) can be signalled by a peripheral or
/// triggered by software. This is the highest priority exception other than
/// reset. It is permanently enabled and has a fixed priority of -2. NMIs
/// cannot be:
/// * masked or prevented from activation by any other exception
/// * preempted by any exception other than Reset.
pub unsafe extern "C" fn isr_nmi() {
    // Do nothing
}

/// A MemManage fault is an exception that occurs because of a memory
/// protection related fault. The the fixed memory protection constraints
/// determines this fault, for both instruction and data memory transactions.
/// This fault is always used to abort instruction accesses to Execute Never
/// (XN) memory regions.
pub unsafe extern "C" fn isr_mmfault() {
    board::panic();
}

/// A BusFault is an exception that occurs because of a memory related fault
/// for an instruction or data memory transaction. This might be from an error
/// detected on a bus in the memory system.
pub unsafe extern "C" fn isr_busfault() {
    board::panic();
}

/// A UsageFault is an exception that occurs because of a fault related to instruction execution. This includes:
/// * an undefined instruction
/// * an illegal unaligned access
/// * invalid state on instruction execution
/// * an error on exception return.
/// The following can cause a UsageFault when the core is configured to report them:
/// * an unaligned address on word and halfword memory access
/// * division by zero.
pub unsafe extern "C" fn isr_usagefault() {
    board::panic();
}

/// A supervisor call (SVC) is an exception that is triggered by the SVC
/// instruction. In an OS environment, applications can use SVC instructions
/// to access OS kernel functions and device drivers.
pub unsafe extern "C" fn isr_svcall() {
    // Nothing
}

/// Debug monitor interrupt handler.
pub unsafe extern "C" fn isr_debugmon() {
    // Nothing
}

/// PendSV is an interrupt-driven request for system-level service. In an OS
/// environment, use PendSV for context switching when no other exception is
/// active.
pub unsafe extern "C" fn isr_pendsv() {
    // Nothing
}

/// A SysTick exception is an exception the system timer generates when it
/// reaches zero. Software can also generate a SysTick exception. In an OS
/// environment, the processor can use this exception as system tick.
pub unsafe extern "C" fn isr_systick() {
    systick::isr();
}

/// A place-holder ISR used when we have nothing better to use.
pub unsafe extern "C" fn isr_empty_def() {
    board::panic();
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
