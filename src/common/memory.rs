//! Implements basic memory allocation. Taken from Phil Opperman's excellent blog
//! http://os.phil-opp.com/kernel-heap.html

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use spin::Mutex;
use linked_list_allocator::Heap;

extern "C" {
    // From the linker script. The address of _heap_bottom is the start of the heap.
    static _heap_bottom: usize;
    // From the linker script. The address of _heap_top is the end of the heap.
    static _heap_top: usize;
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

lazy_static! {
    static ref HEAP: Mutex<Heap> = {
        let start = unsafe { &_heap_bottom as *const _ as usize };
        let end = unsafe { &_heap_top as *const _ as usize };
        let size = (end - start) - 1;
        Mutex::new(unsafe { Heap::new(start, size) })
    };
}

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    HEAP.lock().allocate_first_fit(size, align).expect("out of memory")
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    unsafe { HEAP.lock().deallocate(ptr, size, align) };
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    size: usize,
                                    new_size: usize,
                                    align: usize)
                                    -> *mut u8 {
    use core::{ptr, cmp};

    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8,
                                            size: usize,
                                            _new_size: usize,
                                            _align: usize)
                                            -> usize {
    size
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
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
