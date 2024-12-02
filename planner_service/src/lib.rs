mod err;
use domain::Schedule;
use err::PlannerError;
use general_repository::PlannerRepository;
use uuid::Uuid;
use tracing::error;

struct PlannerService<T: PlannerRepository> {
    repository: T,
}

impl<T: PlannerRepository> PlannerService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn create_schedule(self, user_id: Uuid, class_name: String, schedule: Schedule) -> err::Result<()> {
        self.repository.add_schedule(user_id, class_name, schedule);

        Ok(())
    }
} 

