mod routes;
mod services;
mod models;
mod repositories;
mod handlers;

use std::net::SocketAddr;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use crate::repositories::vet_repository::VetRepository;
use crate::services::vet_service::VetService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();


    let pool = PgPoolOptions::new()
        .max_connections(std::env::var("DATABASE_MAX_CONNECTIONS")?.parse()?)
        .connect(&*std::env::var("DATABASE_URL")?)
        .await?;

    let vet_repo = VetRepository::new(pool.clone());
    let vet_service = VetService::new(vet_repo);

    let app = routes::create_routes(vet_service);

    // Настраиваем TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://{addr}");

    // Запускаем сервер (новый способ)
    axum::serve(listener, app).await?;
    Ok(())
}
