mod err;
pub mod sqlite_db;

use domain::{Class, Schedule, Block};
use mockall::automock;

#[automock]
pub trait PlannerRepository: Send {
    // Class operations
    async fn add_class(&self, user_id: i32, class_name: String) -> err::Result<()>;
    async fn delete_class(&self, user_id: i32, class_id: i32) -> err::Result<()>;
    async fn get_class(&self, user_id: i32, class_id: i32) -> err::Result<Class>;
    async fn get_classes(&self, user_id: i32) -> err::Result<Vec<Class>>;

    // Schedule operations (require class_id)
    async fn add_schedule(&self, class_id: i32, schedule_name: String) -> err::Result<()>;
    async fn delete_schedule(&self, schedule_id: i32) -> err::Result<()>;
    async fn get_schedules(&self, class_id: i32) -> err::Result<Vec<Schedule>>;

    // Block operations
    async fn add_block(&self, schedule_id: i32, block: Block) -> err::Result<()>;
    async fn delete_block(&self, block_id: i32) -> err::Result<()>;
    async fn get_blocks(&self, schedule_id: i32) -> err::Result<Vec<Block>>;

    async fn add_user(&self) -> err::Result<i32>;
}

