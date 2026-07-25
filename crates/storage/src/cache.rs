use std::collections::HashMap;

use parking_lot::RwLock;
use pickledb_core::{
    types::PageId,
};
use pickledb_pages::page::SlottedPage;

/// A simple buffer pool that caches recently used pages in memory.
///
/// Thread-safe via RwLock.
pub struct BufferPool {
    capacity: usize,
    pages: RwLock<HashMap<PageId, CachedPage>>,
    access_order: RwLock<Vec<PageId>>,
}

struct CachedPage {
    data: [u8; 4096],
    dirty: bool,
}

impl BufferPool {
    /// Create a new buffer pool with the given capacity (number of pages).
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            pages: RwLock::new(HashMap::with_capacity(capacity)),
            access_order: RwLock::new(Vec::with_capacity(capacity)),
        }
    }

    /// Retrieve a page from the pool, or None if not cached.
    pub fn get(&self, page_id: PageId) -> Option<SlottedPage> {
        let pages = self.pages.read();
        if let Some(entry) = pages.get(&page_id) {
            let slotted = SlottedPage::from_buffer(entry.data).ok()?;
            drop(pages);
            let mut order = self.access_order.write();
            order.retain(|id| *id != page_id);
            order.push(page_id);
            Some(slotted)
        } else {
            None
        }
    }

    /// Place a page into the cache.
    pub fn put(&self, page: &SlottedPage, dirty: bool) {
        let page_id = page.page_id();
        let data = *page.buffer();
        let mut pages = self.pages.write();
        let mut order = self.access_order.write();

        if pages.len() >= self.capacity && !pages.contains_key(&page_id) {
            if let Some(evict_id) = order.first().copied() {
                pages.remove(&evict_id);
                order.retain(|id| *id != evict_id);
            }
        }

        pages.insert(page_id, CachedPage { data, dirty });
        order.retain(|id| *id != page_id);
        order.push(page_id);
    }

    /// Check if a page is dirty.
    pub fn is_dirty(&self, page_id: PageId) -> bool {
        self.pages.read().get(&page_id).map(|e| e.dirty).unwrap_or(false)
    }

    /// Remove a page from the cache.
    pub fn remove(&self, page_id: PageId) {
        let mut pages = self.pages.write();
        let mut order = self.access_order.write();
        pages.remove(&page_id);
        order.retain(|id| *id != page_id);
    }

    /// Mark a page as clean after flushing.
    pub fn mark_clean(&self, page_id: PageId) {
        if let Some(entry) = self.pages.write().get_mut(&page_id) {
            entry.dirty = false;
        }
    }

    /// Collect all dirty page IDs for flushing.
    pub fn dirty_pages(&self) -> Vec<PageId> {
        self.pages
            .read()
            .iter()
            .filter(|(_, e)| e.dirty)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Current number of cached pages.
    pub fn len(&self) -> usize {
        self.pages.read().len()
    }

    /// Check if cache is empty.
    pub fn is_empty(&self) -> bool {
        self.pages.read().is_empty()
    }

    /// Clear all entries.
    pub fn clear(&self) {
        self.pages.write().clear();
        self.access_order.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pickledb_core::types::PageType;

    fn test_page(id: u32) -> SlottedPage {
        SlottedPage::new(PageType::Data, PageId(id))
    }

    #[test]
    fn test_put_and_get() {
        let pool = BufferPool::new(10);
        let page = test_page(0);
        pool.put(&page, false);
        assert_eq!(pool.len(), 1);
        let retrieved = pool.get(PageId(0));
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_get_missing() {
        let pool = BufferPool::new(10);
        assert!(pool.get(PageId(99)).is_none());
    }

    #[test]
    fn test_eviction() {
        let pool = BufferPool::new(2);
        pool.put(&test_page(0), false);
        pool.put(&test_page(1), false);
        pool.put(&test_page(2), false);
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_dirty_pages() {
        let pool = BufferPool::new(10);
        pool.put(&test_page(0), true);
        pool.put(&test_page(1), false);
        pool.put(&test_page(2), true);
        let dirty = pool.dirty_pages();
        assert!(dirty.contains(&PageId(0)));
        assert!(!dirty.contains(&PageId(1)));
        assert!(dirty.contains(&PageId(2)));
    }

    #[test]
    fn test_mark_clean() {
        let pool = BufferPool::new(10);
        pool.put(&test_page(0), true);
        assert!(pool.is_dirty(PageId(0)));
        pool.mark_clean(PageId(0));
        assert!(!pool.is_dirty(PageId(0)));
    }

    #[test]
    fn test_remove() {
        let pool = BufferPool::new(10);
        pool.put(&test_page(0), false);
        assert_eq!(pool.len(), 1);
        pool.remove(PageId(0));
        assert!(pool.is_empty());
    }

    #[test]
    fn test_clear() {
        let pool = BufferPool::new(10);
        pool.put(&test_page(0), false);
        pool.put(&test_page(1), false);
        pool.clear();
        assert!(pool.is_empty());
    }
}
