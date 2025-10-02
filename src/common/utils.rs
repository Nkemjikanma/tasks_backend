use crate::common::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
pub struct PasswordUtils;

impl PasswordUtils {
    /// Hashes a plain-text password using Argon2.
    ///
    /// This function generates a random salt and uses Argon2 with default parameters
    /// to create a secure hash of the provided password.
    ///
    /// # Arguments
    ///
    /// * `password` - The plain-text password to hash
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The hashed password as a string
    /// * `Err(AppError::PasswordHashingFailed)` - If hashing fails
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHashingFailed)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verfify_passowrd(password: &str, hash: &str) -> bool {
        let password_hash = match PasswordHash::new(&hash) {
            Ok(h) => h,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }
}
