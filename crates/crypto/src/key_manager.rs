use hkdf::Hkdf;
use sha2::Sha256;
use pickledb_core::error::CryptoError;

const INFO_ENC: &[u8] = b"pickledb-enc-key";
const INFO_SEARCH: &[u8] = b"pickledb-search-key";

/// Derives an encryption key and a search key from a master key using HKDF-SHA256.
pub struct KeyManager {
    enc_key: [u8; 32],
    search_key: [u8; 32],
}

impl KeyManager {
    /// Derive K_enc and K_search from K_master using HKDF-SHA256.
    pub fn new(master_key: &[u8]) -> Result<Self, CryptoError> {
        if master_key.is_empty() {
            return Err(CryptoError::InvalidKeyLength(
                "master key must not be empty".into(),
            ));
        }

        let hk = Hkdf::<Sha256>::new(None, master_key);

        let mut enc_key = [0u8; 32];
        hk.expand(INFO_ENC, &mut enc_key)
            .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

        let mut search_key = [0u8; 32];
        hk.expand(INFO_SEARCH, &mut search_key)
            .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

        Ok(Self { enc_key, search_key })
    }

    /// Returns a reference to the derived encryption key.
    pub fn encryption_key(&self) -> &[u8; 32] {
        &self.enc_key
    }

    /// Returns a reference to the derived search key.
    pub fn search_key(&self) -> &[u8; 32] {
        &self.search_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let master = b"this-is-a-test-master-key-32bytes!";
        let km = KeyManager::new(master).unwrap();
        assert_ne!(km.encryption_key(), &[0u8; 32]);
        assert_ne!(km.search_key(), &[0u8; 32]);
    }

    #[test]
    fn test_key_derivation_deterministic() {
        let master = b"same-master-key-1234567890abcdef";
        let km1 = KeyManager::new(master).unwrap();
        let km2 = KeyManager::new(master).unwrap();
        assert_eq!(km1.encryption_key(), km2.encryption_key());
        assert_eq!(km1.search_key(), km2.search_key());
    }

    #[test]
    fn test_key_derivation_different_keys() {
        let master = b"same-master-key-1234567890abcdef";
        let km = KeyManager::new(master).unwrap();
        assert_ne!(km.encryption_key(), km.search_key());
    }

    #[test]
    fn test_empty_master_key() {
        assert!(KeyManager::new(b"").is_err());
    }

    #[test]
    fn test_different_master_different_keys() {
        let km1 = KeyManager::new(b"master-key-one-1234567890abcde").unwrap();
        let km2 = KeyManager::new(b"master-key-two-1234567890abcde").unwrap();
        assert_ne!(km1.encryption_key(), km2.encryption_key());
        assert_ne!(km1.search_key(), km2.search_key());
    }

    #[test]
    fn test_search_token_non_zero() {
        let master = b"test-master-for-search-token-1234";
        let km = KeyManager::new(master).unwrap();
        assert!(!km.search_key().iter().all(|&b| b == 0));
    }
}
