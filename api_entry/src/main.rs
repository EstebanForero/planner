use std::net::SocketAddr;
use axum::Router;
use general_repository::sqlite_db::SqlitePlannerRepository;
use planner_api::planner_router;
mod planner_api;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = SqlitePlannerRepository::generate_pool().await;

    let cors = tower_http::cors::CorsLayer::permissive();

    let planner_router = planner_router(pool);

    let main_router = Router::new()
        .nest("/planner", planner_router)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, main_router).await.unwrap();
}
