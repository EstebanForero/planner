use mockall::automock;
use uuid::Uuid;

#[automock]
trait PlannerRepository {
    async fn add_schedule(user_id: Uuid, class_name: String);
    async fn delete_schedule(user_id: Uuid, class_name: String);
}
