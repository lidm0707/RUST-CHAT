use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginModel {
    email: String,
    password: String,
}

impl LoginModel {
    pub fn new(email: String, password: String) -> Self {
        LoginModel { email, password }
    }
}
