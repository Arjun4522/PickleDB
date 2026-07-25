use pickledb_core::{
    error::WalError,
    types::LSN,
};

use crate::log::{WalEntry, WalLog};

/// Applies replayed WAL entries to restore state after a crash.
///
/// The recovery process:
/// 1. Open the WAL
/// 2. Replay all entries after the last checkpoint
/// 3. Return the operations to apply to the storage engine
pub struct Recovery;

impl Recovery {
    /// Recover state from the WAL, returning the operations to re-apply.
    pub fn recover(wal: &mut WalLog) -> Result<Vec<WalEntry>, WalError> {
        let entries = wal.replay()?;
        Ok(entries)
    }

    /// Recover and return the highest LSN seen.
    pub fn recover_lsn(wal: &mut WalLog) -> Result<LSN, WalError> {
        let entries = wal.replay()?;
        Ok(entries
            .iter()
            .map(|e| e.lsn)
            .max()
            .unwrap_or(LSN(0)))
    }

    /// Check if any operations were lost by comparing expected vs actual LSN.
    pub fn verify_integrity(
        wal: &mut WalLog,
        expected_lsn: LSN,
    ) -> Result<bool, WalError> {
        let entries = wal.replay()?;
        let max_lsn = entries
            .iter()
            .map(|e| e.lsn)
            .max()
            .unwrap_or(LSN(0));
        Ok(max_lsn >= expected_lsn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pickledb_core::types::{EncryptedPayload, InsertTuple, RecordId, SearchToken, WalOperation};
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> String {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir()
            .join(format!("pickledb_recovery_test_{}_{}", std::process::id(), id));
        let _ = fs::create_dir_all(&dir);
        dir.to_string_lossy().to_string()
    }

    fn sample_insert() -> WalOperation {
        let payload = EncryptedPayload {
            nonce: [0u8; 12],
            ciphertext: vec![1u8; 32],
            tag: [2u8; 16],
        };
        WalOperation::Insert(InsertTuple {
            record_id: RecordId(1),
            payload,
            search_tokens: vec![SearchToken([3u8; 32])],
        })
    }

    #[test]
    fn test_recover_empty() {
        let dir = temp_dir();
        let mut wal = WalLog::open(&dir).unwrap();
        let entries = Recovery::recover(&mut wal).unwrap();
        assert!(entries.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_recover_entries() {
        let dir = temp_dir();
        {
            let mut wal = WalLog::open(&dir).unwrap();
            wal.append(sample_insert()).unwrap();
            wal.append(WalOperation::Delete(RecordId(2))).unwrap();
        }
        {
            let mut wal = WalLog::open(&dir).unwrap();
            let entries = Recovery::recover(&mut wal).unwrap();
            assert_eq!(entries.len(), 2);
            match &entries[0].operation {
                WalOperation::Insert(_) => {}
                _ => panic!("expected Insert"),
            }
            match &entries[1].operation {
                WalOperation::Delete(id) => assert_eq!(id.0, 2),
                _ => panic!("expected Delete"),
            }
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_recover_lsn() {
        let dir = temp_dir();
        {
            let mut wal = WalLog::open(&dir).unwrap();
            wal.append(sample_insert()).unwrap();
            wal.append(sample_insert()).unwrap();
            wal.append(sample_insert()).unwrap();
        }
        {
            let mut wal = WalLog::open(&dir).unwrap();
            let lsn = Recovery::recover_lsn(&mut wal).unwrap();
            assert_eq!(lsn, LSN(3));
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_verify_integrity() {
        let dir = temp_dir();
        {
            let mut wal = WalLog::open(&dir).unwrap();
            wal.append(sample_insert()).unwrap();
        }
        {
            let mut wal = WalLog::open(&dir).unwrap();
            assert!(Recovery::verify_integrity(&mut wal, LSN(1)).unwrap());
            assert!(!Recovery::verify_integrity(&mut wal, LSN(2)).unwrap());
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
