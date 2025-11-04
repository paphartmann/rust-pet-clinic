use crate::models::owner::pet::PetWithVisits;

#[derive(Debug, Clone)]
pub struct Owner {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
}

impl Owner {
    pub fn new(
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        phone: Option<String>,
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

#[derive(Debug, Clone)]
pub struct OwnerWithPetsAndVisits {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub pets: Vec<PetWithVisits>,
}

impl OwnerWithPetsAndVisits {
    pub fn new(
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        phone: Option<String>,
        pets: Vec<PetWithVisits>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            address,
            city,
            phone,
            pets,
        }
    }
}
