use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginModel {
    pub user: String,
    pub password: String,
}

impl LoginModel {
    pub fn new(user: String, password: String) -> Self {
        LoginModel { user, password }
    }
}
