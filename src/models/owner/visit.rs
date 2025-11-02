use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Visit {
    pub id: i32,
    pub visit_date: Option<NaiveDate>,
    pub description: Option<String>,
}

impl Visit {
    pub fn new(id: i32, visit_date: Option<NaiveDate>, description: Option<String>) -> Self {
        Self {
            id,
            visit_date,
            description,
        }
    }
}
