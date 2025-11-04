use crate::handlers::owner_handler::PetRequest;
use crate::models::owner::owner::{OwnerAdd, OwnerWithPetsAndVisits};
use crate::models::owner::pet::{PetAdd, PetWithVisits};
use crate::repositories::owner_repository::OwnerRepository;
use crate::repositories::pet_repository::PetRepository;
use crate::repositories::visit_repository::VisitRepository;
use std::collections::HashMap;

#[derive(Clone)]
pub struct OwnerService {
    owner_repository: OwnerRepository,
    pet_repository: PetRepository,
    visit_repository: VisitRepository,
}

impl OwnerService {
    pub fn new(
        owner_repository: OwnerRepository,
        pet_repository: PetRepository,
        visit_repository: VisitRepository,
    ) -> Self {
        Self {
            owner_repository,
            pet_repository,
            visit_repository,
        }
    }

    pub async fn get_owner(&self, owner_id: i32) -> anyhow::Result<Option<OwnerWithPetsAndVisits>> {
        let Some(owner) = self.owner_repository.get_owner(owner_id).await? else {
            return Ok(None);
        };

        let pets = self.pet_repository.find_pets_by_owner_id(&owner_id).await?;
        let pet_ids = pets.iter().map(|p| p.id).collect::<Vec<_>>();
        let visits = self.visit_repository.get_visits(&pet_ids).await?;

        let pet_id_to_visits = visits.into_iter().fold(HashMap::new(), |mut map, visit| {
            map.entry(visit.pet_id).or_insert_with(Vec::new).push(visit);
            map
        });

        let owner_with_pets = OwnerWithPetsAndVisits::new(
            owner_id,
            owner.first_name,
            owner.last_name,
            owner.address,
            owner.city,
            owner.phone,
            pets.into_iter()
                .map(|p| {
                    PetWithVisits::new(
                        p.id,
                        p.name,
                        p.pet_type,
                        p.birth_date,
                        pet_id_to_visits
                            .get(&p.id)
                            .cloned()
                            .unwrap_or_else(Vec::new),
                    )
                })
                .collect(),
        );

        Ok(Option::Some(owner_with_pets))
    }

    pub async fn add_pet(&self, owner_id: i32, pet_request: PetAdd) -> anyhow::Result<i32> {
        Ok(self.pet_repository.add_pet(owner_id, pet_request).await?)
    }

    pub async fn update_pet(
        &self,
        owner_id: i32,
        pet_id: i32,
        pet_request: PetAdd,
    ) -> anyhow::Result<()> {
        self.pet_repository
            .update_pet(owner_id, pet_id, pet_request)
            .await?;

        Ok(())
    }

    pub async fn add_owner(&self, owner_request: OwnerAdd) -> anyhow::Result<i32> {
        Ok(self.owner_repository.add_owner(owner_request).await?)
    }
}
