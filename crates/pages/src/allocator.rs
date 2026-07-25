use pickledb_core::{
    error::PageError,
    types::PageId,
};

/// Manages page ID allocation and a free list for page reuse.
pub struct PageAllocator {
    next_page_id: u32,
    free_list: Vec<PageId>,
}

impl PageAllocator {
    /// Create a new allocator starting from the given next page ID.
    pub fn new(start_page_id: u32) -> Self {
        Self {
            next_page_id: start_page_id,
            free_list: Vec::new(),
        }
    }

    /// Allocate a page ID, either from the free list or by incrementing the counter.
    pub fn allocate(&mut self) -> PageId {
        if let Some(freed) = self.free_list.pop() {
            freed
        } else {
            let id = PageId(self.next_page_id);
            self.next_page_id += 1;
            id
        }
    }

    /// Free a page ID, making it available for reuse.
    pub fn free(&mut self, page_id: PageId) {
        self.free_list.push(page_id);
    }

    /// Returns the number of freed (reusable) page IDs.
    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }

    /// Returns the total number of pages allocated so far (including freed).
    pub fn total_allocated(&self) -> u32 {
        self.next_page_id
    }

    /// Returns the number of actively used page IDs.
    pub fn used_count(&self) -> u32 {
        self.next_page_id - self.free_list.len() as u32
    }

    /// Serialize the free list for persistence.
    pub fn serialize_free_list(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.free_list.len() * 4);
        out.extend_from_slice(&self.next_page_id.to_be_bytes());
        out.extend_from_slice(&(self.free_list.len() as u32).to_be_bytes());
        for pid in &self.free_list {
            out.extend_from_slice(&pid.0.to_be_bytes());
        }
        out
    }

    /// Deserialize the free list from previously serialized data.
    pub fn deserialize_free_list(data: &[u8]) -> Result<Self, PageError> {
        if data.len() < 8 {
            return Err(PageError::CorruptHeader);
        }
        let next = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let count = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
        if data.len() < 8 + count * 4 {
            return Err(PageError::CorruptHeader);
        }
        let mut free_list = Vec::with_capacity(count);
        for i in 0..count {
            let base = 8 + i * 4;
            let id = u32::from_be_bytes([data[base], data[base + 1], data[base + 2], data[base + 3]]);
            free_list.push(PageId(id));
        }
        Ok(Self {
            next_page_id: next,
            free_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_monotonic() {
        let mut alloc = PageAllocator::new(0);
        assert_eq!(alloc.allocate(), PageId(0));
        assert_eq!(alloc.allocate(), PageId(1));
        assert_eq!(alloc.allocate(), PageId(2));
        assert_eq!(alloc.used_count(), 3);
    }

    #[test]
    fn test_free_and_reuse() {
        let mut alloc = PageAllocator::new(0);
        let _p0 = alloc.allocate();
        let p1 = alloc.allocate();
        let _p2 = alloc.allocate();
        alloc.free(p1);
        assert_eq!(alloc.free_count(), 1);
        assert_eq!(alloc.used_count(), 2);
        assert_eq!(alloc.allocate(), p1);
    }

    #[test]
    fn test_free_list_lifo() {
        let mut alloc = PageAllocator::new(10);
        let a = alloc.allocate();
        let b = alloc.allocate();
        let _c = alloc.allocate();
        alloc.free(a);
        alloc.free(b);
        // LIFO: last freed is b
        assert_eq!(alloc.allocate(), b);
        assert_eq!(alloc.allocate(), a);
        assert_eq!(alloc.allocate(), PageId(13));
    }

    #[test]
    fn test_serialize_roundtrip() {
        let mut alloc = PageAllocator::new(5);
        let p0 = alloc.allocate();
        let _p1 = alloc.allocate();
        alloc.free(p0);
        let data = alloc.serialize_free_list();
        let mut restored = PageAllocator::deserialize_free_list(&data).unwrap();
        assert_eq!(restored.next_page_id, 7);
        assert_eq!(restored.free_count(), 1);
        assert_eq!(restored.allocate(), p0);
    }

    #[test]
    fn test_empty_free_list() {
        let alloc = PageAllocator::new(0);
        assert_eq!(alloc.free_count(), 0);
        assert_eq!(alloc.used_count(), 0);
    }

    #[test]
    fn test_serialize_empty() {
        let alloc = PageAllocator::new(7);
        let data = alloc.serialize_free_list();
        let mut restored = PageAllocator::deserialize_free_list(&data).unwrap();
        assert_eq!(restored.allocate(), PageId(7));
    }

    #[test]
    fn test_deserialize_corrupt() {
        assert!(PageAllocator::deserialize_free_list(b"short").is_err());
    }
}
