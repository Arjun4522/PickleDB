use hmac::{Hmac, Mac};
use pickledb_core::types::SearchToken;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Generates search tokens using HMAC-SHA256.
///
/// `token = HMAC(K_search, field_id || "::" || value)`
pub struct TokenGenerator {
    key: [u8; 32],
}

impl TokenGenerator {
    /// Create a new token generator from the 32-byte search key.
    pub fn new(search_key: &[u8; 32]) -> Self {
        Self { key: *search_key }
    }

    /// Derive a search token for the given field name and value.
    pub fn generate(&self, field_id: &str, value: &str) -> SearchToken {
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .expect("HMAC-SHA256 accepts 32-byte keys");

        mac.update(field_id.as_bytes());
        mac.update(b"::");
        mac.update(value.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        let mut token_bytes = [0u8; 32];
        token_bytes.copy_from_slice(&code_bytes);
        SearchToken(token_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_generator() -> TokenGenerator {
        let key = [0x42u8; 32];
        TokenGenerator::new(&key)
    }

    #[test]
    fn test_token_generation() {
        let gen = test_generator();
        let token = gen.generate("email", "alice@example.com");
        assert_ne!(token.as_bytes(), &[0u8; 32]);
    }

    #[test]
    fn test_token_deterministic() {
        let key = [0x99u8; 32];
        let gen1 = TokenGenerator::new(&key);
        let gen2 = TokenGenerator::new(&key);

        let t1 = gen1.generate("email", "alice@example.com");
        let t2 = gen2.generate("email", "alice@example.com");
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_different_field_different_token() {
        let gen = test_generator();
        let t1 = gen.generate("email", "alice@example.com");
        let t2 = gen.generate("name", "alice@example.com");
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_different_value_different_token() {
        let gen = test_generator();
        let t1 = gen.generate("email", "alice@example.com");
        let t2 = gen.generate("email", "bob@example.com");
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_different_key_different_token() {
        let gen1 = TokenGenerator::new(&[0x11u8; 32]);
        let gen2 = TokenGenerator::new(&[0x22u8; 32]);
        let t1 = gen1.generate("email", "a@b.com");
        let t2 = gen2.generate("email", "a@b.com");
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_token_length() {
        let gen = test_generator();
        let token = gen.generate("field", "value");
        assert_eq!(token.as_bytes().len(), 32);
    }

    #[test]
    fn test_empty_field_and_value() {
        let gen = test_generator();
        let t1 = gen.generate("", "value");
        let t2 = gen.generate("field", "");
        let t3 = gen.generate("", "");
        assert_ne!(t1.as_bytes(), &[0u8; 32]);
        assert_ne!(t2.as_bytes(), &[0u8; 32]);
        assert_ne!(t3.as_bytes(), &[0u8; 32]);
        assert_ne!(t1, t2);
        assert_ne!(t1, t3);
        assert_ne!(t2, t3);
    }
}
