use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}

impl LoginModel {
    pub fn new(username: String, password: String) -> Self {
        LoginModel { username, password }
    }
}
