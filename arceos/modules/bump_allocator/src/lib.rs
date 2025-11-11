#![no_std]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end:   usize,
    b_pos: usize,
    p_pos: usize,
    b_count: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            b_count: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = start + size;
        self.b_count = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let size = layout.size();
        let align = layout.align();
        let aligned_b_pos = (self.b_pos + align - 1) & !(align - 1);
        let new_b_pos = aligned_b_pos + size;
        if new_b_pos > self.p_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        self.b_pos = new_b_pos;
        self.b_count += 1;

        NonNull::new(aligned_b_pos as *mut u8).ok_or(AllocError::InvalidParam)
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        if self.b_count > 0 {
            self.b_count -= 1;
            if self.b_count == 0 {
                self.b_pos = self.start;
            }
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        let alloc_size = num_pages * SIZE;
        let align = SIZE * (1 << align_pow2);
        
        let new_p_pos = (self.p_pos - alloc_size) & !(align - 1);
        
        if new_p_pos < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        
        self.p_pos = new_p_pos;
        Ok(new_p_pos)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        // EarlyAllocator does not support deallocation of pages.
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / SIZE
    }

    fn used_pages(&self) -> usize {
        (self.p_pos - self.start) / SIZE
    }

    fn available_pages(&self) -> usize {
        (self.end - self.p_pos) / SIZE
    }
}