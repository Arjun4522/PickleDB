use std::collections::HashMap;

use parking_lot::RwLock;
use pickledb_core::{
    error::EngineError,
    traits::Engine,
    types::{
        EncryptedPayload, InsertTuple, PageId, PageType, RecordId, SearchToken, WalOperation,
    },
};
use pickledb_index::hash_index::HashIndex;
use pickledb_pages::allocator::PageAllocator;
use pickledb_pages::page::SlottedPage;
use pickledb_storage::cache::BufferPool;
use pickledb_storage::manager::FileManager;
use pickledb_wal::log::{WalEntry, WalLog};

/// The core PickleDB storage engine.
///
/// Composes file manager, buffer pool, WAL, index, and page allocator
/// into a single encrypted database engine.
pub struct PickleEngine {
    file_manager: RwLock<FileManager>,
    buffer_pool: RwLock<BufferPool>,
    wal: RwLock<WalLog>,
    index: HashIndex,
    page_allocator: RwLock<PageAllocator>,
    record_map: RwLock<HashMap<RecordId, (PageId, u16)>>,
    record_tokens: RwLock<HashMap<RecordId, Vec<SearchToken>>>,
    _dir: String,
}

impl PickleEngine {
    /// Open or create a database in the given directory.
    pub fn open(dir: &str) -> Result<Self, EngineError> {
        let file_manager = FileManager::open(dir)?;
        let wal = WalLog::open(dir)?;
        let buffer_pool = BufferPool::new(1000);
        let index = HashIndex::new();
        let page_allocator = PageAllocator::new(0);

        let mut engine = Self {
            file_manager: RwLock::new(file_manager),
            buffer_pool: RwLock::new(buffer_pool),
            wal: RwLock::new(wal),
            index,
            page_allocator: RwLock::new(page_allocator),
            record_map: RwLock::new(HashMap::new()),
            record_tokens: RwLock::new(HashMap::new()),
            _dir: dir.to_string(),
        };

        // Recover from WAL
        engine.recover()?;

        Ok(engine)
    }

    /// Recover state from the WAL after a crash.
    fn recover(&mut self) -> Result<(), EngineError> {
        let entries = {
            let mut wal = self.wal.write();
            wal.replay().map_err(|e| EngineError::InvalidOperation(e.to_string()))?
        };

        for entry in &entries {
            self.apply_wal_entry(entry)?;
        }

        self.rebuild_record_map()?;

        Ok(())
    }

    /// Rebuild record_map by scanning all data pages.
    /// After WAL recovery, this ensures record_map is correct.
    fn rebuild_record_map(&self) -> Result<(), EngineError> {
        let fm = self.file_manager.read();
        let num_pages = fm.num_pages();
        drop(fm);

        let mut map = self.record_map.write();

        for pid in 0..num_pages {
            let page_id = PageId(pid as u32);
            let page = match self.file_manager.write().read_page(page_id) {
                Ok(p) => p,
                Err(_) => continue,
            };
            for slot in 0..page.record_count() {
                if let Ok(data) = page.get_record(slot) {
                    if let Ok((stored_id, _)) = bincode::deserialize::<(RecordId, EncryptedPayload)>(data) {
                        // Later entries overwrite earlier ones (newest wins)
                        map.insert(stored_id, (page_id, slot));
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply a single WAL entry to restore engine state.
    ///
    /// Only restores in-memory index state. Actual page data is on disk.
    /// The record_map is rebuilt from page data after recovery.
    fn apply_wal_entry(&self, entry: &WalEntry) -> Result<(), EngineError> {
        match &entry.operation {
            WalOperation::Insert(tuple) => {
                for token in &tuple.search_tokens {
                    self.index
                        .insert(token.clone(), tuple.record_id)
                        .ok();
                }
                self.record_tokens
                    .write()
                    .insert(tuple.record_id, tuple.search_tokens.clone());
                Ok(())
            }
            WalOperation::Delete(record_id) => {
                self.record_map.write().remove(record_id);
                self.record_tokens.write().remove(record_id);
                Ok(())
            }
            WalOperation::Update {
                record_id: _,
                tuple: _,
            } => {
                Ok(())
            }
            WalOperation::Checkpoint(_) => Ok(()),
        }
    }

    /// Find a page with space for a record, or allocate a new one.
    fn find_or_allocate_page(&self, data_size: usize) -> Result<(SlottedPage, PageId), EngineError> {
        let fm = self.file_manager.read();
        let num_pages = fm.num_pages();
        drop(fm);

        for pid in 0..num_pages {
            let page_id = PageId(pid as u32);
            if let Some(cached) = self.buffer_pool.read().get(page_id) {
                if cached.page_type() == PageType::Data && cached.can_fit(data_size) {
                    return Ok((cached, page_id));
                }
            } else {
                let mut fm = self.file_manager.write();
                if let Ok(page) = fm.read_page(page_id) {
                    if page.page_type() == PageType::Data && page.can_fit(data_size) {
                        return Ok((page, page_id));
                    }
                }
            }
        }

        let page_id = self.page_allocator.write().allocate();
        let page = SlottedPage::new(PageType::Data, page_id);
        Ok((page, page_id))
    }
}

impl Engine for PickleEngine {
    fn insert(&mut self, tuple: InsertTuple) -> Result<(), EngineError> {
        // Store (RecordId, EncryptedPayload) so we can rebuild record_map from pages
        let page_entry = (tuple.record_id, &tuple.payload);
        let payload_bytes = bincode::serialize(&page_entry)
            .map_err(|e| EngineError::InvalidOperation(e.to_string()))?;

        // WAL append
        {
            let mut wal = self.wal.write();
            wal.append(WalOperation::Insert(tuple.clone()))?;
        }

        // Find or allocate a page
        let (mut page, page_id) = self.find_or_allocate_page(payload_bytes.len())?;
        let slot = page.insert_record(&payload_bytes)?;

        // Write page to disk and cache
        {
            let mut fm = self.file_manager.write();
            fm.write_page(&page)?;
        }
        self.buffer_pool.write().put(&page, false);

        // Update index
        for token in &tuple.search_tokens {
            self.index.insert(token.clone(), tuple.record_id)?;
        }

        // Track record location
        self.record_map
            .write()
            .insert(tuple.record_id, (page_id, slot));

        // Track tokens for index cleanup on delete
        self.record_tokens
            .write()
            .insert(tuple.record_id, tuple.search_tokens.clone());

        Ok(())
    }

    fn search(&self, token: &SearchToken) -> Result<Vec<RecordId>, EngineError> {
        self.index.search(token).map_err(Into::into)
    }

    fn delete(&mut self, record_id: RecordId) -> Result<(), EngineError> {
        {
            let mut wal = self.wal.write();
            wal.append(WalOperation::Delete(record_id))?;
        }

        // Remove search tokens from index
        if let Some(tokens) = self.record_tokens.write().remove(&record_id) {
            for token in &tokens {
                self.index.delete(token, record_id).ok();
            }
        }

        // Remove from page tracking
        if let Some((page_id, slot)) = self.record_map.write().remove(&record_id) {
            let cached_page = self.buffer_pool.read().get(page_id);
            if let Some(mut cached) = cached_page {
                cached.delete_record(slot).ok();
                self.buffer_pool.write().put(&cached, true);
            } else {
                let mut fm = self.file_manager.write();
                if let Ok(mut page) = fm.read_page(page_id) {
                    page.delete_record(slot).ok();
                    fm.write_page(&page)?;
                }
            }
        }

        Ok(())
    }

    fn update(&mut self, record_id: RecordId, tuple: InsertTuple) -> Result<(), EngineError> {
        // WAL append
        {
            let mut wal = self.wal.write();
            wal.append(WalOperation::Update {
                record_id,
                tuple: tuple.clone(),
            })?;
        }

        // Remove old record location
        self.record_map.write().remove(&record_id);

        // Insert as new record
        self.insert(tuple)
    }

    fn get(&self, record_id: RecordId) -> Result<EncryptedPayload, EngineError> {
        let map = self.record_map.read();
        let (page_id, slot) = map
            .get(&record_id)
            .ok_or(EngineError::RecordNotFound(record_id))?;

        let page = if let Some(cached) = self.buffer_pool.read().get(*page_id) {
            cached
        } else {
            self.file_manager.write().read_page(*page_id)?
        };

        let data = page
            .get_record(*slot)
            .map_err(|_| EngineError::RecordNotFound(record_id))?;

        let (stored_id, payload): (RecordId, EncryptedPayload) = bincode::deserialize(data)
            .map_err(|_| EngineError::InvalidOperation("corrupt record data".into()))?;

        if stored_id != record_id {
            return Err(EngineError::RecordNotFound(record_id));
        }

        Ok(payload)
    }

    fn sync(&mut self) -> Result<(), EngineError> {
        self.file_manager.write().flush()?;
        self.wal.write().sync()?;
        Ok(())
    }

    fn checkpoint(&mut self) -> Result<(), EngineError> {
        let lsn = self.wal.read().current_lsn();
        self.wal.write().checkpoint(lsn)?;
        Ok(self.file_manager.write().flush()?)
    }

    fn compact(&mut self) -> Result<(), EngineError> {
        let fm = self.file_manager.read();
        let num_pages = fm.num_pages();
        drop(fm);

        for pid in 0..num_pages {
            let page_id = PageId(pid as u32);
            let mut page = if let Some(cached) = self.buffer_pool.read().get(page_id) {
                cached
            } else {
                self.file_manager.write().read_page(page_id)?
            };

            page.compact_records();
            self.file_manager.write().write_page(&page)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pickledb_core::types::EncryptedPayload;
    use std::fs;

    fn setup(name: &str) -> (PickleEngine, String) {
        let dir = std::env::temp_dir()
            .join(format!("pickledb_engine_test_{}_{}", std::process::id(), name));
        let _ = fs::create_dir_all(&dir);
        let dir_str = dir.to_string_lossy().to_string();
        let engine = PickleEngine::open(&dir_str).unwrap();
        (engine, dir_str)
    }

    fn sample_payload() -> EncryptedPayload {
        EncryptedPayload {
            nonce: [0u8; 12],
            ciphertext: vec![1u8; 32],
            tag: [2u8; 16],
        }
    }

    fn sample_tuple(record_id: u64, token_val: u8) -> InsertTuple {
        InsertTuple {
            record_id: RecordId(record_id),
            payload: sample_payload(),
            search_tokens: vec![SearchToken([token_val; 32])],
        }
    }

    #[test]
    fn test_open_close() {
        let (_engine, dir) = setup("open_close");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_insert_and_search() {
        let (mut engine, dir) = setup("insert_search");
        let tuple = sample_tuple(1, 10);
        engine.insert(tuple).unwrap();

        let token = SearchToken([10; 32]);
        let results = engine.search(&token).unwrap();
        assert_eq!(results, vec![RecordId(1)]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_insert_and_get() {
        let (mut engine, dir) = setup("insert_get");
        let payload = sample_payload();
        let tuple = InsertTuple {
            record_id: RecordId(1),
            payload: payload.clone(),
            search_tokens: vec![SearchToken([1; 32])],
        };
        engine.insert(tuple).unwrap();

        let retrieved = engine.get(RecordId(1)).unwrap();
        assert_eq!(retrieved.nonce, payload.nonce);
        assert_eq!(retrieved.ciphertext, payload.ciphertext);
        assert_eq!(retrieved.tag, payload.tag);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_search_multiple_tokens() {
        let (mut engine, dir) = setup("multi_token");
        let mut t1 = sample_tuple(1, 10);
        t1.search_tokens.push(SearchToken([20; 32]));
        engine.insert(t1).unwrap();

        let r1 = engine.search(&SearchToken([10; 32])).unwrap();
        let r2 = engine.search(&SearchToken([20; 32])).unwrap();
        assert_eq!(r1, vec![RecordId(1)]);
        assert_eq!(r2, vec![RecordId(1)]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_delete() {
        let (mut engine, dir) = setup("delete");
        engine.insert(sample_tuple(1, 1)).unwrap();
        engine.delete(RecordId(1)).unwrap();

        assert!(engine.get(RecordId(1)).is_err());
        let token = SearchToken([1; 32]);
        match engine.search(&token) {
            Err(EngineError::Index(_)) => {}
            _ => panic!("expected token not found"),
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_update() {
        let (mut engine, dir) = setup("update");
        let old_payload = sample_payload();
        engine
            .insert(InsertTuple {
                record_id: RecordId(1),
                payload: old_payload.clone(),
                search_tokens: vec![SearchToken([1; 32])],
            })
            .unwrap();

        let new_payload = EncryptedPayload {
            nonce: [0xFFu8; 12],
            ciphertext: vec![0xAAu8; 64],
            tag: [0xBBu8; 16],
        };
        engine
            .update(
                RecordId(1),
                InsertTuple {
                    record_id: RecordId(1),
                    payload: new_payload.clone(),
                    search_tokens: vec![SearchToken([2; 32])],
                },
            )
            .unwrap();

        let retrieved = engine.get(RecordId(1)).unwrap();
        assert_eq!(retrieved.ciphertext, new_payload.ciphertext);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_sync_and_checkpoint() {
        let (mut engine, dir) = setup("sync_checkpoint");
        engine.insert(sample_tuple(1, 1)).unwrap();
        engine.sync().unwrap();
        engine.checkpoint().unwrap();
        assert!(engine.get(RecordId(1)).is_ok());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_crash_recovery() {
        let dir = std::env::temp_dir()
            .join(format!("pickledb_engine_crash_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let dir_str = dir.to_string_lossy().to_string();

        {
            let mut engine = PickleEngine::open(&dir_str).unwrap();
            engine.insert(sample_tuple(1, 1)).unwrap();
            engine.insert(sample_tuple(2, 2)).unwrap();
            engine.sync().unwrap();
        }

        // Simulate crash recovery by reopening
        {
            let engine = PickleEngine::open(&dir_str).unwrap();
            assert!(engine.get(RecordId(1)).is_ok());
            assert!(engine.get(RecordId(2)).is_ok());
        }

        let _ = fs::remove_dir_all(&dir_str);
    }
}