mod err;
pub mod sqlite_db;

use domain::{Class, Schedule, Block};
use mockall::automock;

#[automock]
pub trait PlannerRepository: Send {
    // Class operations
    async fn add_class(&self, user_id: i32, class_name: String) -> err::Result<()>;
    async fn delete_class(&self, user_id: i32, class_name: String) -> err::Result<()>;
    async fn get_class(&self, user_id: i32, class_name: String) -> err::Result<Class>;
    async fn get_classes(&self, user_id: i32) -> err::Result<Vec<Class>>;

    // Schedule operations
    async fn add_schedule(&self, class_name: String, schedule: Schedule) -> err::Result<()>;
    async fn delete_schedule(&self, class_name: String, schedule_id: String) -> err::Result<()>;

    // Block operations
    async fn add_block(&self, schedule_id: String, block: Block) -> err::Result<()>;
    async fn delete_block(&self, block_id: i32) -> err::Result<()>;
    async fn get_blocks(&self, schedule_id: String) -> err::Result<Vec<Block>>;
}

