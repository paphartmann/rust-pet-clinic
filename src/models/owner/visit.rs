use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Visit {
    pub id: i32,
    pub visit_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub pet_id: i32,
}

impl Visit {
    pub fn new(
        id: i32,
        visit_date: Option<NaiveDate>,
        description: Option<String>,
        pet_id: i32,
    ) -> Self {
        Self {
            id,
            visit_date,
            description,
            pet_id
        }
    }
}

pub struct VisitAdd{
    pub visit_date: Option<NaiveDate>,
    pub description: Option<String>,
}