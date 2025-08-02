use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use shared::models::claims_model::Claims;

use crate::{
    infrastructures::argon2_infra::verify_password, repositories::auth_repository::UserRepository,
};

pub struct AuthService<T>
where
    T: UserRepository,
{
    pub user_repository: T,
}

impl<T> AuthService<T>
where
    T: UserRepository,
{
    pub fn new(repository: T) -> Self {
        Self {
            user_repository: repository,
        }
    }

    pub fn verify_password(&self, username: &str, password: &str) -> bool {
        let hashed = self.user_repository.get_hashed_password(username);
        // verify_password(password, &hashed);
        true
    }
}
