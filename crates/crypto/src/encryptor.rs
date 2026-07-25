use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Nonce,
};
use pickledb_core::{
    error::CryptoError,
    types::{EncryptedPayload, RecordId},
};
use rand::RngCore;

const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;

/// Encrypts and decrypts payloads using AES-256-GCM.
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// Create a new encryptor from a 32-byte encryption key.
    pub fn new(enc_key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(enc_key)
            .expect("AES-256-GCM requires a 32-byte key");
        Self { cipher }
    }

    /// Encrypt plaintext with a random nonce and RecordId as associated data.
    pub fn encrypt(
        &self,
        record_id: RecordId,
        plaintext: &[u8],
    ) -> Result<EncryptedPayload, CryptoError> {
        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);

        let nonce_for_aead = Nonce::from_slice(&nonce);
        let aad = record_id.0.to_be_bytes();

        let payload = Payload {
            msg: plaintext,
            aad: &aad,
        };

        let ciphertext_with_tag = self
            .cipher
            .encrypt(nonce_for_aead, payload)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        if ciphertext_with_tag.len() < TAG_SIZE {
            return Err(CryptoError::EncryptionFailed(
                "ciphertext too short".into(),
            ));
        }

        let ciphertext_len = ciphertext_with_tag.len() - TAG_SIZE;
        let ciphertext = ciphertext_with_tag[..ciphertext_len].to_vec();
        let mut tag = [0u8; TAG_SIZE];
        tag.copy_from_slice(&ciphertext_with_tag[ciphertext_len..]);

        Ok(EncryptedPayload {
            nonce,
            ciphertext,
            tag,
        })
    }

    /// Decrypt an encrypted payload, verifying the authentication tag.
    pub fn decrypt(
        &self,
        record_id: RecordId,
        payload: &EncryptedPayload,
    ) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(&payload.nonce);
        let aad = record_id.0.to_be_bytes();

        let mut ciphertext_with_tag = payload.ciphertext.clone();
        ciphertext_with_tag.extend_from_slice(&payload.tag);

        let payload_in = Payload {
            msg: &ciphertext_with_tag,
            aad: &aad,
        };

        let plaintext = self
            .cipher
            .decrypt(nonce, payload_in)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_encryptor() -> Encryptor {
        let key = [0xABu8; 32];
        Encryptor::new(&key)
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let enc = test_encryptor();
        let record_id = RecordId(42);
        let plaintext = b"Hello, PickleDB!";

        let payload = enc.encrypt(record_id, plaintext).unwrap();
        assert_eq!(payload.nonce.len(), 12);
        assert_ne!(payload.ciphertext, plaintext);

        let decrypted = enc.decrypt(record_id, &payload).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_different_nonces() {
        let enc = test_encryptor();
        let record_id = RecordId(1);
        let plaintext = b"same data";

        let p1 = enc.encrypt(record_id, plaintext).unwrap();
        let p2 = enc.encrypt(record_id, plaintext).unwrap();
        assert_ne!(p1.nonce, p2.nonce);
        assert_ne!(p1.ciphertext, p2.ciphertext);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = [0xABu8; 32];
        let key2 = [0xCDu8; 32];
        let enc1 = Encryptor::new(&key1);
        let enc2 = Encryptor::new(&key2);

        let record_id = RecordId(0);
        let payload = enc1.encrypt(record_id, b"secret").unwrap();
        assert!(enc2.decrypt(record_id, &payload).is_err());
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails() {
        let enc = test_encryptor();
        let record_id = RecordId(7);
        let mut payload = enc.encrypt(record_id, b"important data").unwrap();
        payload.ciphertext[0] ^= 0xFF;
        assert!(enc.decrypt(record_id, &payload).is_err());
    }

    #[test]
    fn test_decrypt_tampered_tag_fails() {
        let enc = test_encryptor();
        let record_id = RecordId(7);
        let mut payload = enc.encrypt(record_id, b"important data").unwrap();
        payload.tag[0] ^= 0xFF;
        assert!(enc.decrypt(record_id, &payload).is_err());
    }

    #[test]
    fn test_decrypt_wrong_record_id_fails() {
        let enc = test_encryptor();
        let payload = enc.encrypt(RecordId(1), b"data").unwrap();
        assert!(enc.decrypt(RecordId(2), &payload).is_err());
    }

    #[test]
    fn test_large_payload() {
        let enc = test_encryptor();
        let record_id = RecordId(100);
        let plaintext = vec![0x42u8; 10000];

        let payload = enc.encrypt(record_id, &plaintext).unwrap();
        let decrypted = enc.decrypt(record_id, &payload).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_empty_payload() {
        let enc = test_encryptor();
        let record_id = RecordId(0);
        let payload = enc.encrypt(record_id, b"").unwrap();
        let decrypted = enc.decrypt(record_id, &payload).unwrap();
        assert!(decrypted.is_empty());
    }
}
