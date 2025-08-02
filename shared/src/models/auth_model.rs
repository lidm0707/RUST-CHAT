use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthModel {
    pub username: String,
    pub password: String,
}

impl AuthModel {
    pub fn new(username: String, password: String) -> Self {
        AuthModel { username, password }
    }
}
