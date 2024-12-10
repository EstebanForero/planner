use std::env;

use axum::{routing::get, Router};
use general_repository::postgres_db::PostgresPlannerRepository;
use planner_api::planner_router;
mod planner_api;

// Some change in the comments

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let db_url = env::var("PRIVATE_DATABASE_URL").unwrap();
    let pool = PostgresPlannerRepository::generate_pool(&db_url).await;

    let cors = tower_http::cors::CorsLayer::permissive();

    let planner_router = planner_router(pool);

    let main_router = Router::new()
        .nest("/planner", planner_router)
        .route("/", get(health_check))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, main_router).await.unwrap();
}

async fn health_check() -> &'static str {
    "i am alive!!!!!!!!!"
}
