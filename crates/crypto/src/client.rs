use pickledb_core::{
    error::CryptoError,
    traits::Client,
    types::{EncryptedPayload, RecordId, SearchToken},
};

use crate::encryptor::Encryptor;
use crate::key_manager::KeyManager;
use crate::token::TokenGenerator;

/// A concrete implementation of the `Client` trait.
///
/// Manages key derivation, encryption/decryption, and search token generation
/// on the trusted client side. The engine never receives the master key or
/// any derived key material.
pub struct PickleClient {
    encryptor: Encryptor,
    token_gen: TokenGenerator,
}

impl PickleClient {
    /// Create a new client from a master key.
    ///
    /// The master key is used to derive K_enc and K_search via HKDF-SHA256.
    pub fn new(master_key: &[u8]) -> Result<Self, CryptoError> {
        let km = KeyManager::new(master_key)?;
        let encryptor = Encryptor::new(km.encryption_key());
        let token_gen = TokenGenerator::new(km.search_key());
        Ok(Self {
            encryptor,
            token_gen,
        })
    }
}

impl Client for PickleClient {
    fn encrypt(
        &self,
        record_id: RecordId,
        plaintext: &[u8],
    ) -> Result<EncryptedPayload, CryptoError> {
        self.encryptor.encrypt(record_id, plaintext)
    }

    fn decrypt(
        &self,
        record_id: RecordId,
        payload: &EncryptedPayload,
    ) -> Result<Vec<u8>, CryptoError> {
        self.encryptor.decrypt(record_id, payload)
    }

    fn derive_search_token(&self, field_id: &str, value: &str) -> SearchToken {
        self.token_gen.generate(field_id, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> PickleClient {
        PickleClient::new(b"test-master-key-32-bytes-long!!!!!!!").unwrap()
    }

    #[test]
    fn test_client_encrypt_decrypt() {
        let client = test_client();
        let plaintext = b"sensitive data";
        let payload = client.encrypt(RecordId(1), plaintext).unwrap();
        let decrypted = client.decrypt(RecordId(1), &payload).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_client_search_token() {
        let client = test_client();
        let token = client.derive_search_token("email", "alice@example.com");
        assert_eq!(token.as_bytes().len(), 32);
    }

    #[test]
    fn test_client_wrong_record_id_fails() {
        let client = test_client();
        let payload = client.encrypt(RecordId(1), b"data").unwrap();
        assert!(client.decrypt(RecordId(2), &payload).is_err());
    }

    #[test]
    fn test_client_roundtrip_search_and_encrypt() {
        let client = test_client();
        let token = client.derive_search_token("email", "bob@example.com");

        let payload = client.encrypt(RecordId(10), b"Bob's secret").unwrap();
        let decrypted = client.decrypt(RecordId(10), &payload).unwrap();

        assert_eq!(decrypted, b"Bob's secret");
        assert_eq!(token.as_bytes().len(), 32);
    }

    #[test]
    fn test_client_deterministic_tokens() {
        let client = test_client();
        let t1 = client.derive_search_token("field", "value");
        let t2 = client.derive_search_token("field", "value");
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_different_clients_same_key() {
        let c1 = PickleClient::new(b"shared-master-key-1234567890abcdef").unwrap();
        let c2 = PickleClient::new(b"shared-master-key-1234567890abcdef").unwrap();
        let p1 = c1.encrypt(RecordId(0), b"data").unwrap();
        let d2 = c2.decrypt(RecordId(0), &p1).unwrap();
        assert_eq!(d2, b"data");
    }
}
