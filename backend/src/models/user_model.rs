use crate::object_value::{permission::Permission, role::Role};

pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub permissions: Vec<Permission>,
    pub team_id: u32,
}
