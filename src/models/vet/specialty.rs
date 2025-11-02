#[derive(Debug, Clone)]
pub struct Specialty {
    pub id: i32,
    pub name: String,
}

impl Specialty {
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
