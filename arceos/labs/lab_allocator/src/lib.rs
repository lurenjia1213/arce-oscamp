//! Allocator algorithm in lab.
/*
修改说明：把slab的buddy换成了tlsf

有考虑对slab中较小的内存进行回收，但是种种原因最后放弃

看起来tlsf并没有在libc内实现，群友们的实现可能都是slab+buddy

*/
#![no_std]
#![allow(unused_variables)]
#![feature(allocator_api)]
use allocator::{BaseAllocator, ByteAllocator, AllocResult,AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;
mod slab;
//use slab_allocator::Heap;
mod slab_lib;
use slab_lib::Heap;
pub struct LabByteAllocator {
    inner: Option<Heap>,
}
const PAGE_SIZE: usize = 0x1000;
const MIN_HEAP_SIZE: usize = 0x8000; // 32 K

impl LabByteAllocator {
    /// Creates a new empty `SlabByteAllocator`.
    pub const fn new() -> Self {
        Self { inner: None }
    }

    fn inner_mut(&mut self) -> &mut Heap {
        self.inner.as_mut().unwrap()
    }

    fn inner(&self) -> &Heap {
        self.inner.as_ref().unwrap()
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.inner = unsafe { Some(Heap::new(start, size)) };
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe {
            self.inner_mut().add_memory(start, size);
        }
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.inner_mut()
            .allocate(layout)
            .map(|addr| unsafe { NonNull::new_unchecked(addr as *mut u8) })
            .map_err(|_| AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe { self.inner_mut().deallocate(pos.as_ptr() as usize, layout) }
    }

    fn total_bytes(&self) -> usize {
        self.inner().total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.inner().used_bytes()
    }

    fn available_bytes(&self) -> usize {
        self.inner().available_bytes()
    }
}
