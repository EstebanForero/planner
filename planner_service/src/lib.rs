mod err;
use std::collections::HashMap;

use domain::{Block, Class, Schedule};
use err::PlannerError;
use general_repository::PlannerRepository;
use serde::{Deserialize, Serialize};
use tracing::error;

struct PlannerService<T: PlannerRepository> {
    repository: T,
}

impl<T: PlannerRepository> PlannerService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn create_schedule(&self, class_id: i32, schedule_name: String) -> err::Result<()> {
        self.repository.add_schedule(class_id, schedule_name).await.map_err(|_| {
            error!("add schedule has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddScheduleError
        })?;

        Ok(())
    }

    pub async fn remove_schedule(&self, schedule_id: i32) -> err::Result<()> {
        self.repository.delete_schedule(schedule_id).await.map_err(|_| {
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
    
    pub async fn remove_class(&self, user_id: i32, class_id: i32) -> err::Result<()> {
        self.repository.delete_class(user_id, class_id).await.map_err(|_| {
            error!("add class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::RemoveClassError
        })?;

        Ok(())
    }

    pub async fn obtain_class(&self, user_id: i32, class_id: i32) -> err::Result<Class> {
        let class = self.repository.get_class(user_id, class_id).await.map_err(|_| {
            error!("get class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::GetClassError
        })?;

        Ok(class)
    }

    pub async fn obtain_classes(&self, user_id: i32) -> err::Result<Vec<Class>> {
        let classes = self.repository.get_classes(user_id).await.map_err(|_| {
            error!("put the error here");

            PlannerError::GetClassesError
        })?;

        Ok(classes)
    }

    pub async fn generate_plannings(&self, user_id: i32) -> err::Result<Vec<Week>> {
        let classes = self.obtain_classes(user_id).await?;

        let valid_weeks = Vec::new();

        for class in classes {
            let current_week = Week::new();

            for schedule in class.schedules {
                let result = current_week.insert_schedule(&schedule, class.class_name.as_str());

                if let None = result {
                    continue;
                }
            }

            valid_weeks.push(current_week);
        }
    }

    pub fn generate_plans_recursive()
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Week {
    monday: HashMap<u8, HourInfo>,
    tuesday: HashMap<u8, HourInfo>,
    wednesday: HashMap<u8, HourInfo>,
    thursday: HashMap<u8, HourInfo>,
    friday: HashMap<u8, HourInfo>,
    saturday: HashMap<u8, HourInfo>,
}

struct HourInfo {
    class_name: String,
    schedule_name: String
}

impl Week {
    pub fn new() -> Self {
        Self {
            monday: HashMap::new(),
            tuesday: HashMap::new(),
            wednesday: HashMap::new(),
            thursday: HashMap::new(),
            friday: HashMap::new(),
            saturday: HashMap::new()
        }
    }

    pub fn insert_schedule(&mut self, schedule: &Schedule, class_name: &str) -> err::Result<()> {
        for block in &schedule.blocks {
            if self.is_collition(block) {
                return Err(err::PlannerError::AddScheduleError)
            }
        }

        for block in &schedule.blocks {
            match block.day {
                domain::Day::Monday => Week::insert_block_hashmap(&self.monday, block, &schedule.schedule_name, class_name),
                domain::Day::Tuesday => Week::insert_block_hashmap(&self.tuesday, block, &schedule.schedule_name, class_name),
                domain::Day::Wednesday => Week::insert_block_hashmap(&self.wednesday, block, &schedule.schedule_name, class_name),
                domain::Day::Thursday => Week::insert_block_hashmap(&self.thursday, block, &schedule.schedule_name, class_name),
                domain::Day::Friday => Week::insert_block_hashmap(&self.friday, block, &schedule.schedule_name, class_name),
                domain::Day::Saturday => Week::insert_block_hashmap(&self.saturday, block, &schedule.schedule_name, class_name),
            }
        }

        Ok(())
    }

    fn insert_block_hashmap(day_map: &HashMap<u8, HourInfo>, block: &Block, schedule_name: &str, class_name: &str) {
        for hour in block.start_hour..block.finish_hour {
            day_map.insert(hour, HourInfo { 
                schedule_name: schedule_name.to_string(),
                class_name: class_name.to_string() 
            });
        }
    }

    fn is_collition(&self, block: &Block) -> bool {
        match block.day {
            domain::Day::Monday => Week::is_collition_hashmap(&self.monday, block),
            domain::Day::Tuesday => Week::is_collition_hashmap(&self.tuesday, block),
            domain::Day::Wednesday => Week::is_collition_hashmap(&self.wednesday, block),
            domain::Day::Thursday => Week::is_collition_hashmap(&self.thursday, block),
            domain::Day::Friday => Week::is_collition_hashmap(&self.friday, block),
            domain::Day::Saturday => Week::is_collition_hashmap(&self.saturday, block),
        }
    }

    fn is_collition_hashmap(day_map: &HashMap<u8, HourInfo>, block: &Block) -> bool {
        for hour in block.start_hour..block.finish_hour {
            if day_map.contains_key(&hour) {
                return true
            }
        }

        return false
    }
}
