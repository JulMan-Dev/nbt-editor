use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
pub static ALLOCATOR: MallocAllocator = MallocAllocator;

pub struct MallocAllocator;

unsafe impl GlobalAlloc for MallocAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { ::libc::aligned_alloc(layout.align(), layout.size()).cast() }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { ::libc::free(ptr.cast()) }
    }
    
    // C standard doesn't provide aligned realloc and aligned zeroed alloc.
}
