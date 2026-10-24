use super::*;

/// Implements the global allocator contract for the counting allocator.
unsafe impl GlobalAlloc for Counting {
    /// Counts one allocation and hands it to the system allocator.
    ///
    /// # Arguments
    ///
    /// - `Layout` - The layout of the allocation.
    ///
    /// # Returns
    ///
    /// - `*mut u8` - The pointer to the allocated block.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    /// Hands a deallocation back to the system allocator.
    ///
    /// # Arguments
    ///
    /// - `*mut u8` - The pointer to the block.
    /// - `Layout` - The layout of the block.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    /// Counts one reallocation and hands it to the system allocator.
    ///
    /// # Arguments
    ///
    /// - `*mut u8` - The pointer to the block.
    /// - `Layout` - The layout of the block.
    /// - `usize` - The size of the new block.
    ///
    /// # Returns
    ///
    /// - `*mut u8` - The pointer to the reallocated block.
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
