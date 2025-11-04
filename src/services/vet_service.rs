use crate::models::vet::vet::VetWithSpecialties;
use crate::repositories::vet_repository::VetRepository;

#[derive(Clone)]
pub struct VetService {
    repo: VetRepository,
}

impl VetService {
    pub fn new(repo: VetRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_vets(&self) -> anyhow::Result<Vec<VetWithSpecialties>> {
        let vets = self.repo.get_all().await?;

        let vet_specialties = self.repo.get_all_vet_specialty().await?;

        let result = vets
            .into_iter()
            .map(|vet| {
                let specialties = vet_specialties.get(&vet.id).cloned().unwrap_or_default();

                VetWithSpecialties {
                    id: vet.id,
                    first_name: vet.first_name,
                    last_name: vet.last_name,
                    specialties,
                }
            })
            .collect();

        Ok(result)
    }
}
