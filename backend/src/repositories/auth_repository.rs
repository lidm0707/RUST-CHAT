pub trait UserRepository {
    fn new() -> Self;
    fn get_roles(&self, user_id: &str) -> Vec<String>;
    fn get_permissions(&self, user_id: &str) -> Vec<String>;
    fn get_hashed_password(&self, user_id: &str) -> String;
}

pub struct MockUserRepo;

impl UserRepository for MockUserRepo {
    fn new() -> Self {
        Self {}
    }

    fn get_roles(&self, user_id: &str) -> Vec<String> {
        match user_id {
            "1" => vec!["admin".to_string()],
            "2" => vec!["user".to_string()],
            _ => vec![],
        }
    }

    fn get_permissions(&self, user_id: &str) -> Vec<String> {
        match user_id {
            "1" => vec!["edit_user".to_string(), "read_all".to_string()],
            "2" => vec!["read_own".to_string()],
            _ => vec![],
        }
    }

    fn get_hashed_password(&self, user_id: &str) -> String {
        match user_id {
            "1" => "$argon2id$v=19$m=19456,t=2,p=1$BAmAcdzZsNStic+NRnqlcA$R1qrD74cu4J1WS5Qgbne9EGEQcKeEZ9QSBK3gwWPwZg".to_string(),
            "2" => "$argon2id$v=19$m=19456,t=2,p=1$BAmAcdzZsNStic+NRnqlcA$R1qrD74cu4J1WS5Qgbne9EGEQcKeEZ9QSBK3gwWPwZg".to_string(),
            _ => "".to_string(),
        }
    }
}
