//! Handles CPU startup (segment init, etc) that's common to all CPUs.
//! Chip specific init (like the interrupt table) should be in the relevant <chip> module.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use r0;

extern "C" {
    // These symbols come from the linker script
    static mut _data_start_flash: usize;
    static mut _data_start: usize;
    static mut _data_end: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
    // This is defined by your application
    fn launchpad_start();
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

/// Performs what you might otherwise call 'C Startup'.
/// This routine is specified at the reset vector in the ISR vector table.
///
/// Copies global .data init from flash to SRAM and then
/// zeros the bss segment.
#[no_mangle]
pub unsafe extern "C" fn reset_vector() {
    let data_start_flash: *mut usize = &mut _data_start_flash;
    let data_start: *mut usize = &mut _data_start;
    let data_end: *mut usize = &mut _data_end;
    let bss_start: *mut usize = &mut _bss_start;
    let bss_end: *mut usize = &mut _bss_end;

    r0::init_data(data_start, data_end, data_start_flash);
    r0::zero_bss(bss_start, bss_end);

    launchpad_start();
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
