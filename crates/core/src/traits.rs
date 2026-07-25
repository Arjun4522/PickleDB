use crate::error::{
    CacheError, CryptoError, EngineError, IndexError, StorageError, WalError,
};
use crate::types::{
    EncryptedPayload, InsertTuple, Page, PageId, RecordId, SearchToken, WalOperation,
};

/// The core storage engine API.
///
/// All data flowing through this interface is encrypted.
/// The engine never possesses encryption keys or sees plaintext.
pub trait Engine {
    /// Insert a new encrypted record with associated search tokens.
    fn insert(&mut self, tuple: InsertTuple) -> Result<(), EngineError>;

    /// Retrieve all record IDs matching a given search token.
    fn search(&self, token: &SearchToken) -> Result<Vec<RecordId>, EngineError>;

    /// Delete a record by its record ID.
    fn delete(&mut self, record_id: RecordId) -> Result<(), EngineError>;

    /// Update an existing record.
    fn update(&mut self, record_id: RecordId, tuple: InsertTuple) -> Result<(), EngineError>;

    /// Retrieve an encrypted payload by record ID.
    fn get(&self, record_id: RecordId) -> Result<EncryptedPayload, EngineError>;

    /// Flush all pending writes to durable storage.
    fn sync(&mut self) -> Result<(), EngineError>;

    /// Create a checkpoint, trimming the WAL.
    fn checkpoint(&mut self) -> Result<(), EngineError>;

    /// Compact storage, reclaiming space from deleted records.
    fn compact(&mut self) -> Result<(), EngineError>;
}

/// Client-side cryptographic operations.
///
/// Implementations of this trait live in the trusted client and
/// perform all key management and encryption/decryption.
pub trait Client {
    /// Encrypt plaintext for the given record ID.
    fn encrypt(&self, record_id: RecordId, plaintext: &[u8]) -> Result<EncryptedPayload, CryptoError>;

    /// Decrypt an encrypted payload, verifying authenticity.
    fn decrypt(&self, record_id: RecordId, payload: &EncryptedPayload) -> Result<Vec<u8>, CryptoError>;

    /// Derive a search token for a given field name and value.
    fn derive_search_token(&self, field_id: &str, value: &str) -> SearchToken;
}

/// Page storage management.
///
/// Responsible for allocating, reading, writing, and flushing pages.
pub trait PageManager {
    /// Allocate a new page and return its ID.
    fn allocate_page(&mut self) -> Result<PageId, StorageError>;

    /// Read a page by ID.
    fn read_page(&self, page_id: PageId) -> Result<Page, StorageError>;

    /// Write a page to storage.
    fn write_page(&mut self, page: &Page) -> Result<(), StorageError>;

    /// Flush all pending page writes to disk.
    fn flush(&mut self) -> Result<(), StorageError>;
}

/// Write-Ahead Log interface.
pub trait Wal {
    /// Append an operation to the WAL.
    fn append(&mut self, operation: WalOperation) -> Result<LSN, WalError>;

    /// Replay all uncheckpointed WAL entries, recovering to the last consistent state.
    fn replay(&mut self) -> Result<LSN, WalError>;

    /// Truncate the WAL at the current checkpoint position.
    fn checkpoint(&mut self) -> Result<(), WalError>;
}

/// Placeholder for LSN used in WAL trait.
use crate::types::LSN;

/// Blind search index.
///
/// Maps search tokens to sets of matching record IDs.
/// The engine never inspects the token contents.
pub trait Index {
    /// Insert a mapping from a search token to a record ID.
    fn insert(&mut self, token: SearchToken, record_id: RecordId) -> Result<(), IndexError>;

    /// Search for all record IDs matching the given token.
    fn search(&self, token: &SearchToken) -> Result<Vec<RecordId>, IndexError>;

    /// Remove a specific record ID from a token's result set.
    fn delete(&mut self, token: &SearchToken, record_id: RecordId) -> Result<(), IndexError>;
}

/// Page cache / buffer pool.
pub trait Cache {
    /// Retrieve a page from the cache, loading from storage if needed.
    fn get(&mut self, page_id: PageId) -> Result<Page, CacheError>;

    /// Insert or update a page in the cache.
    fn put(&mut self, page: Page) -> Result<(), CacheError>;

    /// Evict a page from the cache.
    fn evict(&mut self) -> Result<(), CacheError>;

    /// Flush all dirty pages to storage.
    fn flush(&mut self) -> Result<(), CacheError>;
}
