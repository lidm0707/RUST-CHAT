pub enum Permission {
    Chat(Execute),
    Settings(Execute),
    Billing(Execute),
    Workflow(Execute),
}

pub struct Execute {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub create: bool,
}

impl Execute {
    pub fn new(read: bool, write: bool, delete: bool, create: bool) -> Self {
        Self {
            read: read,
            write: write,
            delete: delete,
            create: create,
        }
    }
}
