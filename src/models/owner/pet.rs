use crate::models::owner::visit::Visit;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Pet {
    pub id: i32,
    pub name: Option<String>,
    pub pet_type: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

impl Pet {
    pub fn new(
        id: i32,
        name: Option<String>,
        pet_type: Option<String>,
        birth_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            id,
            name,
            pet_type,
            birth_date,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PetWithVisits {
    pub id: i32,
    pub name: Option<String>,
    pub pet_type: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub visits: Vec<Visit>,
}

impl PetWithVisits {
    pub fn new(
        id: i32,
        name: Option<String>,
        pet_type: Option<String>,
        birth_date: Option<NaiveDate>,
        visits: Vec<Visit>,
    ) -> Self {
        Self {
            id,
            name,
            pet_type,
            birth_date,
            visits,
        }
    }
}
