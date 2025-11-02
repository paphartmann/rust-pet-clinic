#[derive(Debug, Clone)]
pub struct Owner {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
}

impl Owner {
    pub fn new(
        id: i32,
        first_name: String,
        last_name: String,
        address: String,
        city: String,
        phone: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            address,
            city,
            phone,
        }
    }
}
