mod err;
use domain::Schedule;
use general_repository::PlannerRepository;
use uuid::Uuid;

struct PlannerService<T: PlannerRepository> {
    repository: T,
}

impl<T: PlannerRepository> PlannerService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }

    pub fn create_schedule(user_id: Uuid, class_name: String, schedule: Schedule) -> err::Result<()> {
        Ok(())
    }
} 
