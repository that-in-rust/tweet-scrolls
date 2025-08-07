//! User anonymization utilities using Blake3 hashing

/// Anonymizes a user ID using Blake3 hashing
/// 
/// This function takes a user ID string and returns a consistent, anonymized hash.
/// The same user ID will always produce the same hash, but the original ID cannot
/// be recovered from the hash.
/// 
/// # Examples
/// 
/// ```
/// use tweet_scrolls::relationship::hash_user_id;
/// 
/// let user_id = "1132151165410455552";
/// let hash1 = hash_user_id(user_id);
/// let hash2 = hash_user_id(user_id);
/// 
/// assert_eq!(hash1, hash2); // Consistent hashing
/// assert_ne!(hash1, user_id); // Actually anonymized
/// assert_eq!(hash1.len(), 64); // Blake3 hash length
/// ```
pub fn hash_user_id(user_id: &str) -> String {
    let hash = blake3::hash(user_id.as_bytes());
    hash.to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_anonymization() {
        let user_id = "1132151165410455552";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);
        
        assert_eq!(hash1, hash2); // Consistent hashing
        assert_ne!(hash1, user_id); // Actually anonymized
        assert_eq!(hash1.len(), 64); // Blake3 hash length
    }

    #[test]
    fn test_user_id_anonymization_different_inputs() {
        let user_id1 = "1132151165410455552";
        let user_id2 = "9876543210123456789";
        
        let hash1 = hash_user_id(user_id1);
        let hash2 = hash_user_id(user_id2);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
        
        // Both should be properly formatted hashes
        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
        
        // Both should be hex strings (only contain 0-9, a-f)
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hash2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_user_id_anonymization_edge_cases() {
        // Test empty string
        let empty_hash = hash_user_id("");
        assert_eq!(empty_hash.len(), 64);
        
        // Test very long string
        let long_id = "a".repeat(1000);
        let long_hash = hash_user_id(&long_id);
        assert_eq!(long_hash.len(), 64);
        
        // Test special characters
        let special_id = "user@123!#$%";
        let special_hash = hash_user_id(special_id);
        assert_eq!(special_hash.len(), 64);
        assert_ne!(special_hash, special_id);
    }

    #[test]
    fn test_hash_consistency() {
        let user_id = "test_user_123";
        let hash1 = hash_user_id(user_id);
        
        // Hash multiple times to ensure consistency
        for _ in 0..10 {
            let hash_n = hash_user_id(user_id);
            assert_eq!(hash1, hash_n, "Hash should be consistent across multiple calls");
        }
    }

    #[test]
    fn test_hash_uniqueness() {
        let mut hashes = std::collections::HashSet::new();
        
        // Generate hashes for different inputs
        for i in 0..100 {
            let user_id = format!("user_{}", i);
            let hash = hash_user_id(&user_id);
            
            // Each hash should be unique
            assert!(hashes.insert(hash), "Hash collision detected for user_{}", i);
        }
        
        assert_eq!(hashes.len(), 100, "Should have 100 unique hashes");
    }
}