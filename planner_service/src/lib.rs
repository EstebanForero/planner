mod err;
use std::collections::HashMap;

use domain::{Class, Schedule};
use err::PlannerError;
use general_repository::PlannerRepository;
use tracing::error;

struct PlannerService<T: PlannerRepository> {
    repository: T,
}

impl<T: PlannerRepository> PlannerService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn create_schedule(&self, user_id: i32, class_name: String, schedule: Schedule) -> err::Result<()> {
        self.repository.add_schedule(user_id, class_name, schedule).await.map_err(|_| {
            error!("add schedule has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddScheduleError
        })?;

        Ok(())
    }

    pub async fn remove_schedule(&self, user_id: i32, class_name: String, schedule_id: String) -> err::Result<()> {
        self.repository.delete_schedule(user_id, class_name, schedule_id).await.map_err(|_| {
            error!("delete schedule has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::RemoveScheduleError
        })?;

        Ok(())
    }

    pub async fn create_class(&self, user_id: i32, class_name: String) -> err::Result<()> {
        self.repository.add_class(user_id, class_name).await.map_err(|_| {
            error!("add class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddClassError
        })?;

        Ok(())
    }
    
    pub async fn remove_class(&self, user_id: i32, class_name: String) -> err::Result<()> {
        self.repository.delete_class(user_id, class_name).await.map_err(|_| {
            error!("add class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::RemoveClassError
        })?;

        Ok(())
    }

    pub async fn obtain_class(&self, user_id: i32, class_name: String) -> err::Result<Class> {
        let class = self.repository.get_class(user_id, class_name).await.map_err(|_| {
            error!("get class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::GetClassError
        })?;

        Ok(class)
    }

    async fn obtain_classes(&self, user_id: i32) -> err::Result<Vec<Class>> {
        let classes = self.repository.get_classes(user_id).await.map_err(|_| {
            error!("put the error here");

            PlannerError::GetClassesError
        })?;

        Ok(classes)
    }
} 

pub struct Week {
    monday: HashMap<u32, String>,
    tuesday: HashMap<u32, String>,
    wednesday: HashMap<u32, String>,
    thursday: HashMap<u32, String>,
    friday: HashMap<u32, String>,
    saturday: HashMap<u32, String>,
}

