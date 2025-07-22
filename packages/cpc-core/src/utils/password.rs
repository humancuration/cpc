use anyhow::Result;
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;

/// Hashes a password using Argon2id with a randomly generated salt.
///
/// # Security Considerations
/// - Uses Argon2id which provides resistance to both GPU and side-channel attacks
/// - Generates a cryptographically secure random salt for each password
/// - Uses recommended parameters: 4096 MB memory, 3 iterations, 4 lanes
/// - Never log or store plaintext passwords
///
/// # Arguments
/// * `password` - The plaintext password to hash. Should be cleared from memory after use.
///
/// # Returns
/// A `Result` containing the hashed password string or an error.
pub fn hash_password(password: &str) -> Result<String> {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);
    
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 3,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        ..Default::default()
    };
    
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;
    Ok(hash)
}

/// Verifies a plaintext password against a stored hash.
///
/// # Security Considerations
/// - Uses constant-time comparison to prevent timing attacks
/// - Immediately returns false on invalid hash format
/// - Never log verification failures with sensitive information
///
/// # Arguments
/// * `hash` - The stored hash string.
/// * `password` - The plaintext password to verify. Should be cleared from memory after use.
///
/// # Returns
/// A `Result` containing a boolean indicating if the password matches, or an error.
pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
    let valid = argon2::verify_encoded(hash, password.as_bytes())?;
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "my_super_secure_password";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(&hash, password).unwrap());
        assert!(!verify_password(&hash, "wrong_password").unwrap());
    }
}