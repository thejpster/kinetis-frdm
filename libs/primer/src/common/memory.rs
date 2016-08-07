//! Implements basic memory allocation.

use spin::Mutex;

extern "C" {
    static _heap_bottom: usize;
    static _heap_top: usize;
}

static BUMP_ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(
    BumpAllocator::new(&_heap_top, &_heap_bottom));

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    BUMP_ALLOCATOR.lock().allocate(size, align).expect("out of memory")
}

#[no_mangle]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize,
    _align: usize)
{
    // just leak it
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
                                align: usize) -> *mut u8 {
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
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize,
    _new_size: usize, _align: usize) -> usize
{
    size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(Debug)]
struct BumpAllocator {
    heap_start: &'static usize,
    heap_end: &'static usize,
    next: usize
}

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

impl BumpAllocator {
    /// Create a new allocator, which uses the memory in the
    /// range [heap_start, heap_end).
    const fn new(heap_start: &'static usize, heap_end: &'static usize) -> BumpAllocator {
        BumpAllocator {
            heap_start: heap_start,
            heap_end: heap_end,
            next: 0,
        }
    }

    /// Allocates a block of memory with the given size and alignment.
    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        if self.next == 0 {
            self.next = self.heap_start as *const usize as usize;
        }

        let alloc_start = align_up(self.next, align);
        let alloc_end = alloc_start + size;
        let heap_end = self.heap_end as *const usize as usize;

        if alloc_end < heap_end {
            self.next = alloc_end;
            Some(alloc_start as *mut u8)
        } else {
            None
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
