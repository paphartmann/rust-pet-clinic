use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Visit {
    pub id: i32,
    pub visit_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub pet_id: i32,
}

pub struct VisitAdd{
    pub visit_date: Option<NaiveDate>,
    pub description: Option<String>,
}