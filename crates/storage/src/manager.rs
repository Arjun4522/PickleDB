use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use pickledb_core::{
    error::StorageError,
    types::{PageId, PAGE_SIZE},
};
use pickledb_pages::page::SlottedPage;

const DATA_FILE: &str = "data.db";

/// Manages on-disk page storage.
///
/// Pages are stored at fixed offsets: `page_id * PAGE_SIZE`.
/// Supports reading, writing, allocating, and flushing pages.
pub struct FileManager {
    data_file: File,
    num_pages: u64,
    _dir: String,
}

impl FileManager {
    /// Open or create the storage files in the given directory.
    pub fn open(dir: &str) -> Result<Self, StorageError> {
        let path = Path::new(dir).join(DATA_FILE);
        let data_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)?;

        let metadata = data_file.metadata()?;
        let num_pages = metadata.len() / (PAGE_SIZE as u64);

        Ok(Self {
            data_file,
            num_pages,
            _dir: dir.to_string(),
        })
    }

    /// Read a page from disk.
    pub fn read_page(&mut self, page_id: PageId) -> Result<SlottedPage, StorageError> {
        let offset = (page_id.0 as u64) * (PAGE_SIZE as u64);
        if offset >= self.data_file.metadata()?.len() {
            return Err(StorageError::PageNotFound(page_id));
        }

        self.data_file.seek(SeekFrom::Start(offset))?;
        let mut buffer = [0u8; PAGE_SIZE];
        self.data_file.read_exact(&mut buffer)?;

        SlottedPage::from_buffer(buffer).map_err(|_| StorageError::CorruptPage(format!("page {}", page_id.0)))
    }

    /// Write a page to disk at the correct offset.
    pub fn write_page(&mut self, page: &SlottedPage) -> Result<(), StorageError> {
        let page_id = page.page_id();
        let offset = (page_id.0 as u64) * (PAGE_SIZE as u64);

        self.data_file.seek(SeekFrom::Start(offset))?;
        self.data_file.write_all(page.buffer())?;

        if page_id.0 as u64 >= self.num_pages {
            self.num_pages = page_id.0 as u64 + 1;
        }

        Ok(())
    }

    /// Allocate a new page and zero it out on disk.
    pub fn allocate_page(&mut self, page_id: PageId) -> Result<SlottedPage, StorageError> {
        let page = SlottedPage::new(pickledb_core::types::PageType::Data, page_id);
        self.write_page(&page)?;
        Ok(page)
    }

    /// Ensure all data is flushed to disk (fsync).
    pub fn flush(&mut self) -> Result<(), StorageError> {
        self.data_file.sync_all()?;
        Ok(())
    }

    /// The number of pages currently on disk.
    pub fn num_pages(&self) -> u64 {
        self.num_pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> String {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir()
            .join(format!("pickledb_storage_test_{}_{}", std::process::id(), id));
        let _ = fs::create_dir_all(&dir);
        dir.to_string_lossy().to_string()
    }

    #[test]
    fn test_open_creates_file() {
        let dir = temp_dir();
        let mut fm = FileManager::open(&dir).unwrap();
        assert_eq!(fm.num_pages(), 0);

        let page = SlottedPage::new(pickledb_core::types::PageType::Data, PageId(0));
        fm.write_page(&page).unwrap();
        fm.flush().unwrap();
        assert_eq!(fm.num_pages(), 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_write_and_read_page() {
        let dir = temp_dir();
        let mut fm = FileManager::open(&dir).unwrap();

        let mut page = SlottedPage::new(pickledb_core::types::PageType::Data, PageId(0));
        page.insert_record(b"test data").unwrap();
        fm.write_page(&page).unwrap();

        let loaded = fm.read_page(PageId(0)).unwrap();
        assert_eq!(loaded.record_count(), 1);
        assert_eq!(loaded.get_record(0).unwrap(), b"test data");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_page_not_found() {
        let dir = temp_dir();
        let mut fm = FileManager::open(&dir).unwrap();
        match fm.read_page(PageId(999)) {
            Err(StorageError::PageNotFound(_)) => {}
            _ => panic!("expected PageNotFound"),
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_multiple_pages() {
        let dir = temp_dir();
        let mut fm = FileManager::open(&dir).unwrap();

        for i in 0..10 {
            let mut page = SlottedPage::new(pickledb_core::types::PageType::Data, PageId(i));
            page.insert_record(format!("record_{}", i).as_bytes()).unwrap();
            fm.write_page(&page).unwrap();
        }
        assert_eq!(fm.num_pages(), 10);

        for i in 0..10 {
            let page = fm.read_page(PageId(i)).unwrap();
            let rec = page.get_record(0).unwrap();
            assert_eq!(rec, format!("record_{}", i).as_bytes());
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
