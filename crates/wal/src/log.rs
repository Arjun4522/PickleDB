use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use pickledb_core::{
    error::WalError,
    types::{LSN, WalOperation},
};

const WAL_FILE: &str = "wal.log";

/// A single WAL entry tied to an LSN.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalEntry {
    pub lsn: LSN,
    pub operation: WalOperation,
}

/// Append-only Write-Ahead Log.
///
/// Format:
/// - First entry begins at offset 0
/// - Each entry: 4-byte length (little-endian u32) + bincode-serialized WalEntry
/// - The file is append-only; entries are never modified in place
pub struct WalLog {
    file: File,
    current_lsn: LSN,
    checkpoint_lsn: LSN,
    entry_count: u64,
}

impl WalLog {
    /// Open or create the WAL file.
    pub fn open(dir: &str) -> Result<Self, WalError> {
        let path = Path::new(dir).join(WAL_FILE);
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)?;

        Ok(Self {
            file,
            current_lsn: LSN(0),
            checkpoint_lsn: LSN(0),
            entry_count: 0,
        })
    }

    /// Append a WAL entry and return its LSN.
    pub fn append(&mut self, operation: WalOperation) -> Result<LSN, WalError> {
        let next_lsn = LSN(self.current_lsn.0 + 1);
        self.current_lsn = next_lsn;
        self.entry_count += 1;

        let entry = WalEntry {
            lsn: next_lsn,
            operation,
        };

        let encoded = bincode::serialize(&entry)
            .map_err(|_e| WalError::CorruptEntry(next_lsn.0))?;

        let len = encoded.len() as u32;
        self.file.seek(SeekFrom::End(0))?;
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(&encoded)?;
        self.file.flush()?;

        Ok(next_lsn)
    }

    /// Replay all entries after the checkpoint LSN.
    ///
    /// Reads from the start of the file, parsing each length-prefixed entry.
    pub fn replay(&mut self) -> Result<Vec<WalEntry>, WalError> {
        let mut entries = Vec::new();
        self.file.seek(SeekFrom::Start(0))?;
        let mut count: u64 = 0;

        loop {
            let mut len_buf = [0u8; 4];
            match self.file.read_exact(&mut len_buf) {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(WalError::Io(e)),
            }

            let entry_len = u32::from_le_bytes(len_buf) as usize;
            let mut entry_buf = vec![0u8; entry_len];
            self.file.read_exact(&mut entry_buf)?;

            let entry: WalEntry = bincode::deserialize(&entry_buf)
                .map_err(|_e| WalError::CorruptEntry(0))?;

            let entry_lsn = entry.lsn;
            count += 1;
            if entry.lsn > self.checkpoint_lsn {
                entries.push(entry);
            }
            if entry_lsn.0 > self.current_lsn.0 {
                self.current_lsn = entry_lsn;
            }
        }

        self.entry_count = count;
        Ok(entries)
    }

    /// Checkpoint: truncate the WAL, keeping only entries after the given LSN.
    pub fn checkpoint(&mut self, lsn: LSN) -> Result<(), WalError> {
        self.checkpoint_lsn = lsn;
        let remaining = self.replay()?;

        // Truncate file and rewrite remaining entries
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;

        for entry in &remaining {
            let encoded = bincode::serialize(entry)
                .map_err(|_| WalError::CorruptEntry(entry.lsn.0))?;
            let len = encoded.len() as u32;
            self.file.write_all(&len.to_le_bytes())?;
            self.file.write_all(&encoded)?;
        }

        self.file.flush()?;
        Ok(())
    }

    /// Get the current LSN.
    pub fn current_lsn(&self) -> LSN {
        self.current_lsn
    }

    /// Get the checkpoint LSN.
    pub fn checkpoint_lsn(&self) -> LSN {
        self.checkpoint_lsn
    }

    /// Sync the WAL to disk.
    pub fn sync(&mut self) -> Result<(), WalError> {
        self.file.flush()?;
        self.file.sync_all()?;
        Ok(())
    }

    /// Return the number of entries in the WAL.
    pub fn entry_count(&self) -> u64 {
        self.entry_count
    }

    /// Verify WAL integrity by replaying and checking for errors.
    pub fn verify_integrity(&mut self) -> Result<(), WalError> {
        self.replay()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pickledb_core::types::{EncryptedPayload, InsertTuple, RecordId, SearchToken};
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> String {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir()
            .join(format!("pickledb_wal_test_{}_{}", std::process::id(), id));
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
    fn test_append_and_replay() {
        let dir = temp_dir();
        let mut wal = WalLog::open(&dir).unwrap();
        assert_eq!(wal.current_lsn(), LSN(0));

        let lsn1 = wal.append(sample_insert()).unwrap();
        assert_eq!(lsn1, LSN(1));
        let lsn2 = wal.append(WalOperation::Delete(RecordId(2))).unwrap();
        assert_eq!(lsn2, LSN(2));

        drop(wal);
        let mut wal2 = WalLog::open(&dir).unwrap();
        assert_eq!(wal2.current_lsn(), LSN(0)); // LSN rebuilt from replay

        let entries = wal2.replay().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].lsn, LSN(1));
        assert_eq!(entries[1].lsn, LSN(2));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_checkpoint() {
        let dir = temp_dir();
        let mut wal = WalLog::open(&dir).unwrap();

        wal.append(sample_insert()).unwrap();
        wal.append(sample_insert()).unwrap();
        wal.checkpoint(LSN(1)).unwrap();

        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].lsn, LSN(2));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_new_wal_empty_replay() {
        let dir = temp_dir();
        let mut wal = WalLog::open(&dir).unwrap();
        let entries = wal.replay().unwrap();
        assert!(entries.is_empty());
        assert_eq!(wal.current_lsn(), LSN(0));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_lsn_monotonic() {
        let dir = temp_dir();
        let mut wal = WalLog::open(&dir).unwrap();
        let l1 = wal.append(sample_insert()).unwrap();
        let l2 = wal.append(sample_insert()).unwrap();
        assert!(l2 > l1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_serde_roundtrip() {
        let operation = sample_insert();
        let entry = WalEntry {
            lsn: LSN(42),
            operation,
        };
        let encoded = bincode::serialize(&entry).unwrap();
        let decoded: WalEntry = bincode::deserialize(&encoded).unwrap();
        assert_eq!(decoded.lsn, LSN(42));
    }

    #[test]
    fn test_replay_recovers_lsn() {
        let dir = temp_dir();
        {
            let mut wal = WalLog::open(&dir).unwrap();
            wal.append(sample_insert()).unwrap();
            wal.append(sample_insert()).unwrap();
            wal.append(sample_insert()).unwrap();
        }
        {
            let mut wal = WalLog::open(&dir).unwrap();
            let _ = wal.replay().unwrap();
            assert_eq!(wal.current_lsn(), LSN(3));
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
