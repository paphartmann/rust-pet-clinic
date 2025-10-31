use crate::repositories::vet_repository::VetRepository;
use crate::models::vet::Vet;

#[derive(Clone)]
pub struct VetService {
    repo: VetRepository,
}

impl VetService {
    pub fn new(repo: VetRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_vets(&self) -> anyhow::Result<Vec<Vet>> {
        self.repo
            .get_all()
            .await
    }
}
