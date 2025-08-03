use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: usize,
}

impl Claims {
    pub fn new(username: String) -> Self {
        Claims {
            sub: username,
            roles: vec![],
            permissions: vec![],
            exp: 0,
        }
    }
}
