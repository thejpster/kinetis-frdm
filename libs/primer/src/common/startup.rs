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
