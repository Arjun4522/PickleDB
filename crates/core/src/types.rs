use std::fmt;

use serde::{Deserialize, Serialize};

/// A unique record identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordId(pub u64);

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RecordId({})", self.0)
    }
}

/// A unique page identifier within a storage file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(pub u32);

impl fmt::Display for PageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PageId({})", self.0)
    }
}

/// Log sequence number used for WAL ordering and recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LSN(pub u64);

impl fmt::Display for LSN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LSN({})", self.0)
    }
}

/// A search token used for blind search.
///
/// The engine never inspects the contents of this token.
/// Tokens are generated client-side via HMAC over a field/value pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SearchToken(pub [u8; 32]);

impl fmt::Display for SearchToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SearchToken({} bytes)", self.0.len())
    }
}

impl SearchToken {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// An encrypted payload stored by the engine.
///
/// Contains all data needed for decryption: nonce, ciphertext, and
/// authentication tag. The engine treats this as opaque bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
    pub tag: [u8; 16],
}

/// An insert tuple sent from the trusted client to the untrusted engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertTuple {
    pub record_id: RecordId,
    pub payload: EncryptedPayload,
    pub search_tokens: Vec<SearchToken>,
}

/// The type of a page in the storage file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageType {
    Data,
    Index,
    Meta,
    Free,
}

/// A slot entry in a slotted page.
///
/// Each slot points to a record stored from the bottom of the page upward,
/// while slots grow downward from the page header.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Slot {
    pub offset: u16,
    pub length: u16,
}

/// Header of a slotted page (fixed-size prefix on every page).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageHeader {
    pub magic: u32,
    pub page_type: PageType,
    pub page_id: PageId,
    pub lsn: LSN,
    pub slot_count: u16,
    pub free_offset: u16,
}

/// A fixed-size 4096-byte page composed of a header, slot array, and data region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub header: PageHeader,
    pub slots: Vec<Slot>,
    pub data: Vec<u8>,
}

/// A WAL operation recorded in the append-only log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalOperation {
    Insert(InsertTuple),
    Delete(RecordId),
    Update { record_id: RecordId, tuple: InsertTuple },
    Checkpoint(LSN),
}

pub const PAGE_SIZE: usize = 4096;
pub const MAGIC_NUMBER: u32 = 0x504B4442; // "PKDB"
