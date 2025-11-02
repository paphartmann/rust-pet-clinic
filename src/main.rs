mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::repositories::vet_repository::VetRepository;
use crate::repositories::visit_repository::VisitRepository;
use crate::services::vet_service::VetService;
use crate::services::visit_service::VisitService;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(std::env::var("DATABASE_MAX_CONNECTIONS")?.parse()?)
        .connect(&*std::env::var("DATABASE_URL")?)
        .await?;

    let vet_repo = VetRepository::new(pool.clone());
    let vet_service = VetService::new(vet_repo);
    let visit_repo = VisitRepository::new(pool.clone());
    let visit_service = VisitService::new(visit_repo);

    let app = routes::create_routes(vet_service, visit_service);

    // Настраиваем TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://{addr}");

    // Запускаем сервер (новый способ)
    axum::serve(listener, app).await?;
    Ok(())
}
