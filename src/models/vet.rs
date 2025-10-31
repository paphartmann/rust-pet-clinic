#[derive(Debug, Clone)]
pub struct Vet {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Vet {
    pub fn new(id: i32, first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            id,
            first_name: first_name.into(),
            last_name: last_name.into(),
        }
    }
}
