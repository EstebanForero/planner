mod err;
pub mod sqlite_db;

use domain::{Class, Schedule};
use mockall::automock;
use uuid::Uuid;

#[automock]
pub trait PlannerRepository: Send {
    async fn add_schedule(&self, user_id: Uuid, class_name: String, schedule: Schedule) -> err::Result<()>;
    async fn delete_schedule(&self, user_id: Uuid, class_name: String, schedule_id: Uuid) -> err::Result<()>;

    async fn add_class(&self, user_id: Uuid, class_name: String) -> err::Result<()>;
    async fn delete_class(&self, user_id: Uuid, class_name: String) -> err::Result<()>;
    async fn get_class(&self, user_id: Uuid, class_name: String) -> err::Result<Class>;
}
