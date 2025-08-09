use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng); // secure random salt
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("hashing failed")
        .to_string(); // convert to string for storage

    hash
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed).expect("Invalid hash format");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
