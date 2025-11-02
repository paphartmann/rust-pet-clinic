use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub pet_type: String,
    pub birthday: NaiveDate,
}

impl Pet {
    pub fn new(id: i32, name: String, pet_type: String, birthday: NaiveDate) -> Self {
        Self {
            id,
            name,
            pet_type,
            birthday,
        }
    }
}
