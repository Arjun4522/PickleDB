use crate::types::*;

#[test]
fn test_record_id() {
    let id = RecordId(42);
    assert_eq!(id.0, 42);
    assert_eq!(format!("{:?}", id), "RecordId(42)");
}

#[test]
fn test_page_id() {
    let id = PageId(1);
    assert_eq!(id.0, 1);
}

#[test]
fn test_lsn_ordering() {
    let a = LSN(10);
    let b = LSN(20);
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, a);
}

#[test]
fn test_search_token() {
    let token = SearchToken([0u8; 32]);
    assert_eq!(token.as_bytes().len(), 32);
}

#[test]
fn test_encrypted_payload() {
    let payload = EncryptedPayload {
        nonce: [1u8; 12],
        ciphertext: vec![2u8; 64],
        tag: [3u8; 16],
    };
    assert_eq!(payload.nonce.len(), 12);
    assert_eq!(payload.ciphertext.len(), 64);
    assert_eq!(payload.tag.len(), 16);
}

#[test]
fn test_insert_tuple() {
    let payload = EncryptedPayload {
        nonce: [0u8; 12],
        ciphertext: vec![0u8; 32],
        tag: [0u8; 16],
    };
    let token = SearchToken([1u8; 32]);
    let tuple = InsertTuple {
        record_id: RecordId(1),
        payload,
        search_tokens: vec![token],
    };
    assert_eq!(tuple.record_id.0, 1);
    assert_eq!(tuple.search_tokens.len(), 1);
}

#[test]
fn test_page_type_serialization() {
    let types = vec![PageType::Data, PageType::Index, PageType::Meta, PageType::Free];
    for pt in types {
        let encoded = bincode::serialize(&pt).unwrap();
        let decoded: PageType = bincode::deserialize(&encoded).unwrap();
        assert_eq!(pt, decoded);
    }
}

#[test]
fn test_page_header() {
    let header = PageHeader {
        magic: MAGIC_NUMBER,
        page_type: PageType::Data,
        page_id: PageId(0),
        lsn: LSN(1),
        slot_count: 0,
        free_offset: std::mem::size_of::<PageHeader>() as u16,
    };
    assert_eq!(header.magic, MAGIC_NUMBER);
    assert_eq!(header.slot_count, 0);
}

#[test]
fn test_wal_operation_roundtrip() {
    let payload = EncryptedPayload {
        nonce: [0u8; 12],
        ciphertext: vec![0u8; 16],
        tag: [0u8; 16],
    };
    let tuple = InsertTuple {
        record_id: RecordId(1),
        payload,
        search_tokens: vec![],
    };
    let op = WalOperation::Insert(tuple);
    let encoded = bincode::serialize(&op).unwrap();
    let decoded: WalOperation = bincode::deserialize(&encoded).unwrap();
    match decoded {
        WalOperation::Insert(t) => assert_eq!(t.record_id.0, 1),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn test_page_size_constant() {
    assert_eq!(PAGE_SIZE, 4096);
}

#[test]
fn test_magic_number() {
    assert_eq!(MAGIC_NUMBER, 0x504B4442);
}
