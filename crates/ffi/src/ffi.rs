use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;

use pickledb_core::{
    traits::Engine,
    types::{EncryptedPayload, InsertTuple, RecordId, SearchToken},
};
use pickledb_engine::engine::PickleEngine;

/// Opaque handle to a PickleDB database instance.
#[repr(C)]
pub struct pickledb_t {
    engine: Mutex<PickleEngine>,
}

/// Result type returned by FFI functions.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum pickledb_result_t {
    PICKLEDB_OK = 0,
    PICKLEDB_ERROR = -1,
    PICKLEDB_NOT_FOUND = -2,
}

/// Open a PickleDB database at the given directory.
///
/// Returns a pointer to a `pickledb_t` instance, or NULL on failure.
/// The returned handle must be freed with `pickledb_close`.
#[no_mangle]
pub extern "C" fn pickledb_open(dir: *const c_char) -> *mut pickledb_t {
    if dir.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(dir) };
    let dir_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match PickleEngine::open(dir_str) {
        Ok(engine) => {
            let handle = pickledb_t {
                engine: Mutex::new(engine),
            };
            Box::into_raw(Box::new(handle))
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Close a PickleDB database and free all resources.
#[no_mangle]
pub extern "C" fn pickledb_close(db: *mut pickledb_t) {
    if !db.is_null() {
        unsafe {
            let _ = Box::from_raw(db);
        }
    }
}

/// Insert a record into the database.
///
/// Returns `PICKLEDB_OK` on success, `PICKLEDB_ERROR` on failure.
#[no_mangle]
pub extern "C" fn pickledb_insert(
    db: *mut pickledb_t,
    record_id: u64,
    data: *const u8,
    data_len: usize,
    token: *const u8,
    token_len: usize,
) -> pickledb_result_t {
    if db.is_null() {
        return pickledb_result_t::PICKLEDB_ERROR;
    }

    let ciphertext = if data.is_null() {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(data, data_len).to_vec() }
    };

    let search_tokens = if !token.is_null() && token_len == 32 {
        let mut token_bytes = [0u8; 32];
        unsafe {
            std::ptr::copy_nonoverlapping(token, token_bytes.as_mut_ptr(), 32);
        }
        vec![SearchToken(token_bytes)]
    } else {
        vec![]
    };

    let payload = EncryptedPayload {
        nonce: [0u8; 12],
        ciphertext,
        tag: [0u8; 16],
    };

    let tuple = InsertTuple {
        record_id: RecordId(record_id),
        payload,
        search_tokens,
    };

    let db_ref = unsafe { &*db };
    match db_ref.engine.lock() {
        Ok(mut engine) => match engine.insert(tuple) {
            Ok(_) => pickledb_result_t::PICKLEDB_OK,
            Err(_) => pickledb_result_t::PICKLEDB_ERROR,
        },
        Err(_) => pickledb_result_t::PICKLEDB_ERROR,
    }
}

/// Search for records matching a search token.
///
/// Returns the number of matching records, or -1 on error.
/// Results are written to `out_ids` and `out_count` if not NULL.
#[no_mangle]
pub extern "C" fn pickledb_search(
    db: *mut pickledb_t,
    token: *const u8,
    token_len: usize,
    out_ids: *mut u64,
    out_count: *mut usize,
) -> i64 {
    if db.is_null() || token.is_null() || token_len != 32 {
        return -1;
    }

    let mut token_bytes = [0u8; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(token, token_bytes.as_mut_ptr(), 32);
    }
    let search_token = SearchToken(token_bytes);

    let db_ref = unsafe { &*db };
    match db_ref.engine.lock() {
        Ok(engine) => match engine.search(&search_token) {
            Ok(records) => {
                let count = records.len();
                if !out_ids.is_null() && !out_count.is_null() {
                    let max_out = unsafe { *out_count };
                    let to_copy = count.min(max_out);
                    for (i, id) in records.iter().take(to_copy).enumerate() {
                        unsafe {
                            *out_ids.add(i) = id.0;
                        }
                    }
                    unsafe {
                        *out_count = to_copy;
                    }
                }
                count as i64
            }
            Err(_) => -1,
        },
        Err(_) => -1,
    }
}

/// Get a record by its ID.
///
/// Returns the encrypted payload via the output pointer, or `PICKLEDB_NOT_FOUND`.
#[no_mangle]
pub extern "C" fn pickledb_get(
    db: *mut pickledb_t,
    record_id: u64,
    out_data: *mut u8,
    out_data_len: *mut usize,
) -> pickledb_result_t {
    if db.is_null() {
        return pickledb_result_t::PICKLEDB_ERROR;
    }

    let db_ref = unsafe { &*db };
    match db_ref.engine.lock() {
        Ok(engine) => match engine.get(RecordId(record_id)) {
            Ok(payload) => {
                if !out_data.is_null() && !out_data_len.is_null() {
                    let max_len = unsafe { *out_data_len };
                    let to_copy = payload.ciphertext.len().min(max_len);
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.ciphertext.as_ptr(),
                            out_data,
                            to_copy,
                        );
                        *out_data_len = to_copy;
                    }
                }
                pickledb_result_t::PICKLEDB_OK
            }
            Err(_) => pickledb_result_t::PICKLEDB_NOT_FOUND,
        },
        Err(_) => pickledb_result_t::PICKLEDB_ERROR,
    }
}

/// Sync all pending writes to durable storage.
#[no_mangle]
pub extern "C" fn pickledb_sync(db: *mut pickledb_t) -> pickledb_result_t {
    if db.is_null() {
        return pickledb_result_t::PICKLEDB_ERROR;
    }

    let db_ref = unsafe { &*db };
    match db_ref.engine.lock() {
        Ok(mut engine) => match engine.sync() {
            Ok(_) => pickledb_result_t::PICKLEDB_OK,
            Err(_) => pickledb_result_t::PICKLEDB_ERROR,
        },
        Err(_) => pickledb_result_t::PICKLEDB_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs;

    fn test_dir() -> CString {
        let dir = std::env::temp_dir()
            .join(format!("pickledb_ffi_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        CString::new(dir.to_string_lossy().as_bytes()).unwrap()
    }

    #[test]
    fn test_open_close() {
        let dir = test_dir();
        let db = pickledb_open(dir.as_ptr());
        assert!(!db.is_null());
        pickledb_close(db);
        let _ = fs::remove_dir_all(dir.to_str().unwrap());
    }

    #[test]
    fn test_insert_and_get() {
        let dir = test_dir();
        let db = pickledb_open(dir.as_ptr());
        assert!(!db.is_null());

        let data = b"test data";
        let result = pickledb_insert(db, 1, data.as_ptr(), data.len(), std::ptr::null(), 0);
        assert_eq!(result, pickledb_result_t::PICKLEDB_OK);

        let mut out_data = [0u8; 64];
        let mut out_len = out_data.len();
        let result = pickledb_get(db, 1, out_data.as_mut_ptr(), &mut out_len);
        assert_eq!(result, pickledb_result_t::PICKLEDB_OK);

        pickledb_close(db);
        let _ = fs::remove_dir_all(dir.to_str().unwrap());
    }

    #[test]
    fn test_get_not_found() {
        let dir = test_dir();
        let db = pickledb_open(dir.as_ptr());
        assert!(!db.is_null());

        let mut out_data = [0u8; 64];
        let mut out_len = out_data.len();
        let result = pickledb_get(db, 999, out_data.as_mut_ptr(), &mut out_len);
        assert_eq!(result, pickledb_result_t::PICKLEDB_NOT_FOUND);

        pickledb_close(db);
        let _ = fs::remove_dir_all(dir.to_str().unwrap());
    }

    #[test]
    fn test_null_handling() {
        assert_eq!(
            pickledb_insert(std::ptr::null_mut(), 0, std::ptr::null(), 0, std::ptr::null(), 0),
            pickledb_result_t::PICKLEDB_ERROR
        );
        assert_eq!(
            pickledb_get(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut()),
            pickledb_result_t::PICKLEDB_ERROR
        );
        assert_eq!(
            pickledb_sync(std::ptr::null_mut()),
            pickledb_result_t::PICKLEDB_ERROR
        );
    }
}
