use pickledb_core::{
    error::PageError,
    types::{LSN, MAGIC_NUMBER, PAGE_SIZE, PageId, PageType},
};

/// Slotted page header size in bytes.
const HEADER_SIZE: usize = 21;
/// Size of each slot entry in bytes.
const SLOT_SIZE: usize = 4;

/// A slotted page managing a fixed-size 4096-byte buffer.
///
/// Layout:
/// - Bytes 0..HEADER_SIZE: page header
/// - Bytes HEADER_SIZE..slot_region_end: slot array (grows downward)
/// - Bytes data_region_start..PAGE_SIZE: record data (grows upward)
pub struct SlottedPage {
    buffer: [u8; PAGE_SIZE],
}

impl SlottedPage {
    /// Create a new empty slotted page.
    pub fn new(page_type: PageType, page_id: PageId) -> Self {
        let mut page = Self {
            buffer: [0u8; PAGE_SIZE],
        };
        page.set_magic(MAGIC_NUMBER);
        page.set_page_type(page_type);
        page.set_page_id(page_id);
        page.set_lsn(LSN(0));
        page.set_slot_count(0);
        page.set_free_offset(PAGE_SIZE as u16);
        page
    }

    /// Create from an existing raw buffer (for loading from disk).
    pub fn from_buffer(buffer: [u8; PAGE_SIZE]) -> Result<Self, PageError> {
        let page = Self { buffer };
        if page.magic() != MAGIC_NUMBER {
            return Err(PageError::MagicMismatch {
                expected: MAGIC_NUMBER,
                actual: page.magic(),
            });
        }
        Ok(page)
    }

    /// Return a reference to the raw buffer.
    pub fn buffer(&self) -> &[u8; PAGE_SIZE] {
        &self.buffer
    }

    /// Consume and return the raw buffer.
    pub fn into_buffer(self) -> [u8; PAGE_SIZE] {
        self.buffer
    }

    // ─── Header accessors ───

    fn magic(&self) -> u32 {
        u32::from_be_bytes([
            self.buffer[0],
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
        ])
    }

    fn set_magic(&mut self, val: u32) {
        self.buffer[0..4].copy_from_slice(&val.to_be_bytes());
    }

    fn page_type_byte(&self) -> u8 {
        self.buffer[4]
    }

    fn set_page_type_byte(&mut self, val: u8) {
        self.buffer[4] = val;
    }

    /// Get the page type.
    pub fn page_type(&self) -> PageType {
        match self.page_type_byte() {
            0 => PageType::Data,
            1 => PageType::Index,
            2 => PageType::Meta,
            3 => PageType::Free,
            _ => PageType::Free,
        }
    }

    /// Set the page type.
    pub fn set_page_type(&mut self, pt: PageType) {
        self.set_page_type_byte(match pt {
            PageType::Data => 0,
            PageType::Index => 1,
            PageType::Meta => 2,
            PageType::Free => 3,
        });
    }

    /// Get the page ID.
    pub fn page_id(&self) -> PageId {
        PageId(u32::from_be_bytes([
            self.buffer[5],
            self.buffer[6],
            self.buffer[7],
            self.buffer[8],
        ]))
    }

    /// Set the page ID.
    pub fn set_page_id(&mut self, id: PageId) {
        self.buffer[5..9].copy_from_slice(&id.0.to_be_bytes());
    }

    /// Get the LSN.
    pub fn lsn(&self) -> LSN {
        LSN(u64::from_be_bytes([
            self.buffer[9],
            self.buffer[10],
            self.buffer[11],
            self.buffer[12],
            self.buffer[13],
            self.buffer[14],
            self.buffer[15],
            self.buffer[16],
        ]))
    }

    /// Set the LSN.
    pub fn set_lsn(&mut self, lsn: LSN) {
        self.buffer[9..17].copy_from_slice(&lsn.0.to_be_bytes());
    }

    /// Get the number of slots.
    pub fn slot_count(&self) -> u16 {
        u16::from_be_bytes([self.buffer[17], self.buffer[18]])
    }

    fn set_slot_count(&mut self, count: u16) {
        self.buffer[17..19].copy_from_slice(&count.to_be_bytes());
    }

    /// Get the free space offset (start of free region from bottom).
    pub fn free_offset(&self) -> u16 {
        u16::from_be_bytes([self.buffer[19], self.buffer[20]])
    }

    fn set_free_offset(&mut self, offset: u16) {
        self.buffer[19..21].copy_from_slice(&offset.to_be_bytes());
    }

    // ─── Slot access ───

    fn slot_end_offset(&self) -> u16 {
        (HEADER_SIZE + (self.slot_count() as usize) * SLOT_SIZE) as u16
    }

    fn read_slot(&self, index: u16) -> Result<(u16, u16), PageError> {
        if index >= self.slot_count() {
            return Err(PageError::InvalidSlot(index));
        }
        let base = HEADER_SIZE + (index as usize) * SLOT_SIZE;
        let offset = u16::from_be_bytes([self.buffer[base], self.buffer[base + 1]]);
        let length = u16::from_be_bytes([self.buffer[base + 2], self.buffer[base + 3]]);
        Ok((offset, length))
    }

    fn write_slot(&mut self, index: u16, offset: u16, length: u16) {
        let base = HEADER_SIZE + (index as usize) * SLOT_SIZE;
        self.buffer[base..base + 2].copy_from_slice(&offset.to_be_bytes());
        self.buffer[base + 2..base + 4].copy_from_slice(&length.to_be_bytes());
    }

    // ─── Public operations ───

    /// Returns the amount of free space available in the page.
    pub fn free_space(&self) -> usize {
        let free_offset = self.free_offset() as usize;
        let slot_end = self.slot_end_offset() as usize;
        if slot_end > free_offset {
            0
        } else {
            free_offset - slot_end
        }
    }

    /// Check if a record of the given size can fit in this page.
    pub fn can_fit(&self, record_size: usize) -> bool {
        self.free_space() >= record_size
    }

    /// Insert a record into the page. Returns the slot index.
    pub fn insert_record(&mut self, data: &[u8]) -> Result<u16, PageError> {
        let record_len = data.len();
        if !self.can_fit(record_len) {
            return Err(PageError::PageFull);
        }

        let slot_idx = self.slot_count();
        let new_free_offset = self.free_offset() - record_len as u16;

        // Write data at the new free offset position
        let data_start = new_free_offset as usize;
        self.buffer[data_start..data_start + record_len].copy_from_slice(data);

        // Add the slot
        self.set_free_offset(new_free_offset);
        self.set_slot_count(slot_idx + 1);
        self.write_slot(slot_idx, new_free_offset, record_len as u16);

        Ok(slot_idx)
    }

    /// Read a record by slot index. Returns the record data.
    pub fn get_record(&self, slot_index: u16) -> Result<&[u8], PageError> {
        let (offset, length) = self.read_slot(slot_index)?;
        let start = offset as usize;
        let end = start + length as usize;
        Ok(&self.buffer[start..end])
    }

    /// Delete a record by slot index (marks slot as unused by shifting).
    ///
    /// This is a naive compaction-free deletion. Use `compact_records` to
    /// reclaim space.
    pub fn delete_record(&mut self, slot_index: u16) -> Result<(), PageError> {
        if slot_index >= self.slot_count() {
            return Err(PageError::InvalidSlot(slot_index));
        }

        let count = self.slot_count();
        // Shift all subsequent slots down by one
        for i in slot_index..count - 1 {
            let (off, len) = self.read_slot(i + 1)?;
            self.write_slot(i, off, len);
        }
        // Zero out the last slot
        let last_slot_start = HEADER_SIZE + ((count - 1) as usize) * SLOT_SIZE;
        self.buffer[last_slot_start..last_slot_start + SLOT_SIZE].fill(0);
        self.set_slot_count(count - 1);

        Ok(())
    }

    /// Update a record in-place (only works if new data fits in same slot).
    pub fn update_record(&mut self, slot_index: u16, data: &[u8]) -> Result<(), PageError> {
        let (offset, length) = self.read_slot(slot_index)?;
        if data.len() > length as usize {
            return Err(PageError::PageFull);
        }
        let start = offset as usize;
        self.buffer[start..start + data.len()].copy_from_slice(data);
        if data.len() < length as usize {
            self.buffer[start + data.len()..start + length as usize].fill(0);
        }
        self.write_slot(slot_index, offset, data.len() as u16);
        Ok(())
    }

    /// Compact the page, defragmenting data and reusing space.
    ///
    /// Returns the number of bytes reclaimed.
    pub fn compact_records(&mut self) -> usize {
        let count = self.slot_count();
        if count == 0 {
            return 0;
        }

        // Collect all record data in order
        let mut records: Vec<Vec<u8>> = Vec::with_capacity(count as usize);
        for i in 0..count {
            let (offset, length) = self.read_slot(i).unwrap();
            let start = offset as usize;
            let end = start + length as usize;
            records.push(self.buffer[start..end].to_vec());
        }

        // Sort records by slot index (already in order)
        // Re-pack from the bottom
        let mut cursor = PAGE_SIZE;
        for (i, rec) in records.iter().enumerate() {
            cursor -= rec.len();
            self.buffer[cursor..cursor + rec.len()].copy_from_slice(rec);
            self.write_slot(i as u16, cursor as u16, rec.len() as u16);
        }

        let old_free_offset = self.free_offset();
        let new_free_offset = cursor as u16;
        let reclaimed = if new_free_offset > old_free_offset {
            (new_free_offset - old_free_offset) as usize
        } else {
            (old_free_offset - new_free_offset) as usize
        };
        self.set_free_offset(new_free_offset);
        reclaimed
    }

    /// Returns the number of slots (records) in this page.
    pub fn record_count(&self) -> u16 {
        self.slot_count()
    }

    /// Check if the page is empty.
    pub fn is_empty(&self) -> bool {
        self.slot_count() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_page() -> SlottedPage {
        SlottedPage::new(PageType::Data, PageId(1))
    }

    #[test]
    fn test_new_page() {
        let page = test_page();
        assert_eq!(page.magic(), MAGIC_NUMBER);
        assert_eq!(page.page_type(), PageType::Data);
        assert_eq!(page.page_id(), PageId(1));
        assert_eq!(page.lsn(), LSN(0));
        assert_eq!(page.slot_count(), 0);
        assert_eq!(page.free_space(), PAGE_SIZE - HEADER_SIZE);
        assert!(page.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut page = test_page();
        let data = b"Hello, PickleDB!";
        let slot = page.insert_record(data).unwrap();
        assert_eq!(slot, 0);
        assert_eq!(page.record_count(), 1);
        assert!(!page.is_empty());
        let retrieved = page.get_record(slot).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_multiple_inserts() {
        let mut page = test_page();
        let d1 = page.insert_record(b"record1").unwrap();
        let d2 = page.insert_record(b"record2").unwrap();
        let d3 = page.insert_record(b"record3").unwrap();
        assert_eq!(d1, 0);
        assert_eq!(d2, 1);
        assert_eq!(d3, 2);
        assert_eq!(page.record_count(), 3);
        assert_eq!(page.get_record(0).unwrap(), b"record1");
        assert_eq!(page.get_record(1).unwrap(), b"record2");
        assert_eq!(page.get_record(2).unwrap(), b"record3");
    }

    #[test]
    fn test_delete_record() {
        let mut page = test_page();
        page.insert_record(b"record1").unwrap();
        page.insert_record(b"record2").unwrap();
        page.insert_record(b"record3").unwrap();
        page.delete_record(1).unwrap();
        assert_eq!(page.record_count(), 2);
        assert_eq!(page.get_record(0).unwrap(), b"record1");
        assert_eq!(page.get_record(1).unwrap(), b"record3");
    }

    #[test]
    fn test_delete_invalid_slot() {
        let mut page = test_page();
        assert!(page.delete_record(0).is_err());
    }

    #[test]
    fn test_update_record() {
        let mut page = test_page();
        page.insert_record(b"original").unwrap();
        page.update_record(0, b"updated").unwrap();
        assert_eq!(page.get_record(0).unwrap(), b"updated");
    }

    #[test]
    fn test_update_too_large() {
        let mut page = test_page();
        page.insert_record(b"small").unwrap();
        assert!(page.update_record(0, b"this is too large for the slot").is_err());
    }

    #[test]
    fn test_compact_records() {
        let mut page = test_page();
        page.insert_record(b"AAAA").unwrap();
        page.insert_record(b"BBBB").unwrap();
        page.insert_record(b"CCCC").unwrap();
        let before_free = page.free_space();
        page.delete_record(1).unwrap();
        page.compact_records();
        // After compaction, records should be contiguous
        assert_eq!(page.get_record(0).unwrap(), b"AAAA");
        assert_eq!(page.get_record(1).unwrap(), b"CCCC");
        assert!(page.free_space() > before_free);
    }

    #[test]
    fn test_page_full() {
        let mut page = test_page();
        let big_data = vec![0u8; PAGE_SIZE - HEADER_SIZE + 1];
        assert!(page.insert_record(&big_data).is_err());
    }

    #[test]
    fn test_many_small_records() {
        let mut page = test_page();
        let mut count = 0u16;
        while page.can_fit(8) {
            page.insert_record(b"12345678").unwrap();
            count += 1;
        }
        assert!(count >= 300);
    }

    #[test]
    fn test_buffer_roundtrip() {
        let mut page = test_page();
        page.insert_record(b"data1").unwrap();
        page.insert_record(b"data2").unwrap();
        page.set_lsn(LSN(42));

        let buffer = page.into_buffer();
        let loaded = SlottedPage::from_buffer(buffer).unwrap();
        assert_eq!(loaded.page_id(), PageId(1));
        assert_eq!(loaded.lsn(), LSN(42));
        assert_eq!(loaded.record_count(), 2);
        assert_eq!(loaded.get_record(0).unwrap(), b"data1");
        assert_eq!(loaded.get_record(1).unwrap(), b"data2");
    }

    #[test]
    fn test_invalid_magic() {
        let mut buffer = [0u8; PAGE_SIZE];
        buffer[0..4].copy_from_slice(&0xDEADBEEFu32.to_be_bytes());
        assert!(SlottedPage::from_buffer(buffer).is_err());
    }

    #[test]
    fn test_delete_all_records() {
        let mut page = test_page();
        page.insert_record(b"only").unwrap();
        page.delete_record(0).unwrap();
        assert!(page.is_empty());
        assert_eq!(page.record_count(), 0);
    }

    #[test]
    fn test_compact_empty_page() {
        let mut page = test_page();
        assert_eq!(page.compact_records(), 0);
    }

    #[test]
    fn test_large_record_fits() {
        let mut page = test_page();
        let data = vec![0xABu8; PAGE_SIZE - HEADER_SIZE - SLOT_SIZE];
        let slot = page.insert_record(&data).unwrap();
        assert_eq!(slot, 0);
        let retrieved = page.get_record(0).unwrap();
        assert_eq!(retrieved.len(), data.len());
        assert_eq!(retrieved, &data[..]);
    }
}
