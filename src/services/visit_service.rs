use crate::models::owner::visit::Visit;
use crate::repositories::visit_repository::VisitRepository;

#[derive(Clone)]
pub struct VisitService {
    repo: VisitRepository,
}

impl VisitService {
    pub fn new(repo: VisitRepository) -> Self {
        Self { repo }
    }

    pub async fn get_visits(&self, pet_id: i32) -> anyhow::Result<Vec<Visit>> {
        let visits = self.repo.get_visits(pet_id).await?;

        Ok(visits)
    }
}
