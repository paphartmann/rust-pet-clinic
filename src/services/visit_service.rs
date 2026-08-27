use crate::models::owner::visit::{Visit, VisitAdd};
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
        let pets_ids = vec![pet_id];

        let visits = self.repo.get_visits(&pets_ids).await?;

        Ok(visits)
    }

    pub async fn add_visit(&self, pet_id: i32, visit_request: VisitAdd) -> anyhow::Result<()> {
        self.repo.add_visit(pet_id, visit_request).await?;

        Ok(())
    }
}
