use uuid::Uuid;


trait PlannerRepository {
    async fn add_schedule(user_id: Uuid);
}
