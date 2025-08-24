pub struct CustomerModel {
    pub id: i32, // room = id customer
    pub name: String,
    pub email: String,
    pub tel: String,
    pub detail: String,
}

impl CustomerModel {
    pub fn new(id: i32, name: String, email: String) -> Self {
        CustomerModel {
            id,
            name,
            email,
            tel: String::new(),
            detail: String::new(),
        }
    }
}
