use std::collections::HashMap;

use parking_lot::RwLock;
use pickledb_core::{
    error::IndexError,
    types::{RecordId, SearchToken},
};

/// An in-memory blind search index backed by a hash map.
///
/// Maps `SearchToken -> Vec<RecordId>`.
/// The engine never inspects token contents.
pub struct HashIndex {
    map: RwLock<HashMap<SearchToken, Vec<RecordId>>>,
}

impl HashIndex {
    /// Create a new empty hash index.
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new hash index with the given pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: RwLock::new(HashMap::with_capacity(capacity)),
        }
    }

    /// Insert a mapping from a search token to a record ID.
    pub fn insert(&self, token: SearchToken, record_id: RecordId) -> Result<(), IndexError> {
        let mut map = self.map.write();
        map.entry(token)
            .or_insert_with(Vec::new)
            .push(record_id);
        Ok(())
    }

    /// Search for all record IDs matching the given token.
    pub fn search(&self, token: &SearchToken) -> Result<Vec<RecordId>, IndexError> {
        let map = self.map.read();
        map.get(token)
            .cloned()
            .ok_or(IndexError::TokenNotFound)
    }

    /// Remove a specific record ID from a token's result set.
    ///
    /// If the record list becomes empty, the token entry is removed.
    pub fn delete(&self, token: &SearchToken, record_id: RecordId) -> Result<(), IndexError> {
        let mut map = self.map.write();
        if let Some(records) = map.get_mut(token) {
            records.retain(|id| *id != record_id);
            if records.is_empty() {
                map.remove(token);
            }
            Ok(())
        } else {
            Err(IndexError::TokenNotFound)
        }
    }

    /// Check if a token has any matching records.
    pub fn contains_token(&self, token: &SearchToken) -> bool {
        self.map.read().contains_key(token)
    }

    /// Return the total number of unique tokens.
    pub fn token_count(&self) -> usize {
        self.map.read().len()
    }

    /// Return the total number of (token, record_id) pairs.
    pub fn total_entries(&self) -> usize {
        self.map.read().values().map(|v| v.len()).sum()
    }

    /// Remove all entries.
    pub fn clear(&self) {
        self.map.write().clear();
    }
}

impl Default for HashIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(val: u8) -> SearchToken {
        SearchToken([val; 32])
    }

    #[test]
    fn test_insert_and_search() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(10)).unwrap();
        let results = idx.search(&token(1)).unwrap();
        assert_eq!(results, vec![RecordId(10)]);
    }

    #[test]
    fn test_search_missing_token() {
        let idx = HashIndex::new();
        match idx.search(&token(99)) {
            Err(IndexError::TokenNotFound) => {}
            _ => panic!("expected TokenNotFound"),
        }
    }

    #[test]
    fn test_multiple_records_per_token() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(1)).unwrap();
        idx.insert(token(1), RecordId(2)).unwrap();
        idx.insert(token(1), RecordId(3)).unwrap();
        let results = idx.search(&token(1)).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains(&RecordId(1)));
        assert!(results.contains(&RecordId(2)));
        assert!(results.contains(&RecordId(3)));
    }

    #[test]
    fn test_delete_record() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(1)).unwrap();
        idx.insert(token(1), RecordId(2)).unwrap();
        idx.delete(&token(1), RecordId(1)).unwrap();
        let results = idx.search(&token(1)).unwrap();
        assert_eq!(results, vec![RecordId(2)]);
    }

    #[test]
    fn test_delete_last_record_removes_token() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(1)).unwrap();
        idx.delete(&token(1), RecordId(1)).unwrap();
        assert!(!idx.contains_token(&token(1)));
    }

    #[test]
    fn test_delete_nonexistent_token() {
        let idx = HashIndex::new();
        match idx.delete(&token(99), RecordId(1)) {
            Err(IndexError::TokenNotFound) => {}
            _ => panic!("expected TokenNotFound"),
        }
    }

    #[test]
    fn test_multiple_tokens() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(10)).unwrap();
        idx.insert(token(2), RecordId(20)).unwrap();
        assert_eq!(idx.token_count(), 2);
        assert_eq!(idx.total_entries(), 2);
    }

    #[test]
    fn test_clear() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(1)).unwrap();
        idx.insert(token(2), RecordId(2)).unwrap();
        idx.clear();
        assert_eq!(idx.token_count(), 0);
    }

    #[test]
    fn test_duplicate_record_same_token() {
        let idx = HashIndex::new();
        idx.insert(token(1), RecordId(1)).unwrap();
        idx.insert(token(1), RecordId(1)).unwrap();
        let results = idx.search(&token(1)).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_with_capacity() {
        let idx = HashIndex::with_capacity(100);
        assert_eq!(idx.token_count(), 0);
        idx.insert(token(0), RecordId(0)).unwrap();
        assert_eq!(idx.token_count(), 1);
    }

    #[test]
    fn test_concurrent_access() {
        let idx = std::sync::Arc::new(HashIndex::new());
        let mut handles = Vec::new();

        for i in 0..10 {
            let idx = idx.clone();
            handles.push(std::thread::spawn(move || {
                idx.insert(token(i as u8), RecordId(i as u64)).unwrap();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(idx.token_count(), 10);
    }
}
