use thiserror::Error;
use crate::types::{PageId, RecordId};

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("key derivation failed: {0}")]
    KeyDerivationFailed(String),
    #[error("invalid key length: {0}")]
    InvalidKeyLength(String),
    #[error("crypto backend error: {0}")]
    Backend(String),
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("page not found: {0}")]
    PageNotFound(PageId),
    #[error("corrupt page: {0}")]
    CorruptPage(String),
    #[error("checksum mismatch")]
    ChecksumMismatch,
    #[error("storage full")]
    StorageFull,
    #[error("invalid page id: {0}")]
    InvalidPageId(u32),
}

#[derive(Error, Debug)]
pub enum PageError {
    #[error("invalid page size: {0}")]
    InvalidPageSize(usize),
    #[error("page full")]
    PageFull,
    #[error("invalid slot index: {0}")]
    InvalidSlot(u16),
    #[error("corrupt page header")]
    CorruptHeader,
    #[error("magic number mismatch: expected {expected:#x}, got {actual:#x}")]
    MagicMismatch { expected: u32, actual: u32 },
    #[error("data region overflow")]
    DataOverflow,
}

#[derive(Error, Debug)]
pub enum WalError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("corrupt WAL entry at offset {0}")]
    CorruptEntry(u64),
    #[error("checksum mismatch in WAL entry")]
    ChecksumMismatch,
    #[error("replay failed at LSN {0}: {1}")]
    ReplayFailed(u64, String),
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("token not found")]
    TokenNotFound,
    #[error("duplicate token entry for record {0}")]
    DuplicateEntry(RecordId),
    #[error("corrupt index")]
    CorruptIndex,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("page not cached: {0}")]
    PageNotCached(PageId),
    #[error("cache full")]
    CacheFull,
    #[error("page is pinned: {0}")]
    PagePinned(PageId),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("page error: {0}")]
    Page(#[from] PageError),
    #[error("WAL error: {0}")]
    Wal(#[from] WalError),
    #[error("index error: {0}")]
    Index(#[from] IndexError),
    #[error("cache error: {0}")]
    Cache(#[from] CacheError),
    #[error("record not found: {0}")]
    RecordNotFound(RecordId),
    #[error("duplicate record: {0}")]
    DuplicateRecord(RecordId),
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),
}
