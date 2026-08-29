mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::repositories::owner_repository::OwnerRepository;
use crate::repositories::pet_repository::PetRepository;
use crate::repositories::vet_repository::VetRepository;
use crate::repositories::visit_repository::VisitRepository;
use crate::services::owner_service::OwnerService;
use crate::services::vet_service::VetService;
use crate::services::visit_service::VisitService;
use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let router = setup_router().await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running at http://{addr}");

    axum::serve(listener, router).await?;
    Ok(())
}

async fn setup_router() -> anyhow::Result<Router> {
    let pool = PgPoolOptions::new()
        .max_connections(std::env::var("DATABASE_MAX_CONNECTIONS")?.parse()?)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let vet_repo = VetRepository::new(pool.clone());
    let vet_service = VetService::new(vet_repo);
    let visit_repo = VisitRepository::new(pool.clone());
    let visit_service = VisitService::new(visit_repo.clone());
    let owner_repo = OwnerRepository::new(pool.clone());
    let pet_repo = PetRepository::new(pool.clone());
    let owner_service = OwnerService::new(owner_repo, pet_repo, visit_repo.clone());

    let app = routes::create_routes(vet_service, visit_service, owner_service);

    Ok(app)
}
