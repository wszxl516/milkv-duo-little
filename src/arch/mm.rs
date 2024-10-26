use lazy_static::lazy_static;
use linked_list_allocator::LockedHeap;

extern "C" {
    fn _heap_start();
    fn _heap_end();
}
lazy_static! {
    pub static ref HEAP_START: usize = _heap_start as usize;
    pub static ref HEAP_END: usize = _heap_end as usize;
}
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    let mem_size = crate::config::MEM_SIZE - (*HEAP_START);
    let heap_start = (*HEAP_START) as *mut u8;
    unsafe {
        ALLOCATOR.lock().init(heap_start, mem_size);
    }
}
