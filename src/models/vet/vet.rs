#[derive(Debug, Clone)]
pub struct Vet {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone)]
pub struct VetWithSpecialties {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub specialties: Vec<String>,
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

impl VetWithSpecialties {
    pub fn new(
        id: i32,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        specialties: Vec<String>,
    ) -> Self {
        Self {
            id,
            first_name: first_name.into(),
            last_name: last_name.into(),
            specialties: specialties,
        }
    }
}
