mod err;

use domain::{Class, Schedule};
use mockall::automock;
use uuid::Uuid;

#[automock]
pub trait PlannerRepository: Send {
    async fn add_schedule(user_id: Uuid, class_name: String, schedule: Schedule);
    async fn delete_schedule(user_id: Uuid, class_name: String, schedule_id: Uuid);

    async fn add_class(user_id: Uuid, class_name: String);
    async fn delete_class(user_id: Uuid, class_name: String);
    async fn get_class(user_id: Uuid, class_name: String) -> err::Result<Class>;
}
