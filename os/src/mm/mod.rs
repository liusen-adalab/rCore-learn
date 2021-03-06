mod address;
mod frame_allocator;
mod heap_allocator;
pub mod memory_set;
pub mod page_table;

pub use address::{PhysAddr, PhysPageNum, StepByOne, VPNRange, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, FrameTracker, frame_dealloc};
pub use memory_set::{MapArea, MapPermission, MemorySet, KERNEL_SPACE, kernel_token};
pub use page_table::{
    translated_byte_buffer,
    translated_refmut,
    translated_str,
    translated_ref,
    PTEFlags,
    PageTable,
    PageTableEntry,
    UserBuffer, 
    UserBufferIterator,
};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}
