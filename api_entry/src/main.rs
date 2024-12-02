
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    general_repository::sqlite_db::SqlitePlannerRepository::generate_pool().await;
}
