mod err;
use std::{collections::HashMap, usize};

use domain::{Block, BlockInfo, Class, RankingParameters, Schedule};
use err::PlannerError;
use general_repository::PlannerRepository;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub struct PlannerService<T: PlannerRepository> {
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
        self.repository.delete_schedule(schedule_id).await.map_err(|err| {
            error!("delete schedule has en error: {}", err);

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
        self.repository.delete_class(user_id, class_id).await.map_err(|err| {
            error!("remove class has en error: {}", err);

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
            error!("get class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::GetClassesError
        })?;

        Ok(classes)
    }

    pub async fn obtain_classes_id(&self, user_id: i32) -> err::Result<Vec<i32>> {
        let classes_id = self.repository.get_classes_id(user_id).await.map_err(|err| {
            error!("Error in get classes id: {}", err);

            PlannerError::GetClassesIdError
        })?;

        Ok(classes_id)
    }

    pub async fn rank_plannings(&self, user_id: i32, ranking_parameters: RankingParameters) -> err::Result<Vec<RatedWeek>> {
        let classes = self.obtain_classes(user_id).await?;

        let mut valid_weeks: Vec<Week> = Vec::new();

        generate_plans_recursive(&mut valid_weeks, 0, &classes, Week::new());

        let mut ranked_weeks: Vec<RatedWeek> = valid_weeks.into_iter().map(|week| {
            let mut total_ranking = 0.;

            total_ranking += get_week_ranking(&week, ranking_parameters.cost_day);
            total_ranking += get_hour_ranking(&week, ranking_parameters.cost_hour);

            RatedWeek {
                week,
                puntuation: total_ranking
            }
        }).collect();

        info!("ranked weeks is: {:?}", ranked_weeks);

        ranked_weeks.sort_unstable_by_key(|ranked_week| (ranked_week.puntuation * 100.) as i32);

        info!("ranked weeks after sort is: {:?}", ranked_weeks);

        Ok(ranked_weeks.into_iter().rev().collect())
    }

    pub async fn generate_plannings(&self, user_id: i32) -> err::Result<Vec<Week>> {
        let classes = self.obtain_classes(user_id).await?;

        let mut valid_weeks: Vec<Week> = Vec::new();

        generate_plans_recursive(&mut valid_weeks, 0, &classes, Week::new());

        Ok(valid_weeks)
    }

    pub async fn add_block(&self, schedule_id: i32, block: BlockInfo) -> err::Result<()> {
        self.repository.add_block(schedule_id, block).await.map_err(|_| {
            error!("get class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddBlockError
        })?;

        Ok(())
    }

    pub async fn delete_block(&self, block_id: i32) -> err::Result<()> {
        self.repository.delete_block(block_id).await.map_err(|err| {
            error!("delete block class has en error: {}", err);

            PlannerError::DeleteBlockError
        })?;

        Ok(())
    }

    pub async fn get_blocks(&self, schedule_id: i32) -> err::Result<Vec<Block>> {
        let blocks = self.repository.get_blocks(schedule_id).await.map_err(|_| {
            error!("get class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::GetBlocksError
        })?;

        Ok(blocks)
    }

    pub async fn create_user(&self) -> err::Result<i32> {
        let id = self.repository.add_user().await.map_err(|_| {
            error!("add user has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddUserError
        })?;

        Ok(id)
    }
} 

fn get_week_ranking(week: &Week, day_points: f32) -> f32 {
    let mut day_total_points = 0.;
    day_total_points -= if week.monday.is_empty() { 0. } else { day_points };
    day_total_points -= if week.tuesday.is_empty() { 0. } else { day_points };
    day_total_points -= if week.wednesday.is_empty() { 0. } else { day_points };
    day_total_points -= if week.thursday.is_empty() { 0. } else { day_points };
    day_total_points -= if week.friday.is_empty() { 0. } else { day_points };
    day_total_points -= if week.saturday.is_empty() { 0. } else { day_points };

    day_total_points
}

fn get_hour_ranking(week: &Week, hour_points: f32) -> f32 {
    let mut hour_total_ranking = 0.;

    hour_total_ranking += get_hour_day_ranking(&week.monday, hour_points);
    hour_total_ranking += get_hour_day_ranking(&week.tuesday, hour_points);
    hour_total_ranking += get_hour_day_ranking(&week.wednesday, hour_points);
    hour_total_ranking += get_hour_day_ranking(&week.thursday, hour_points);
    hour_total_ranking += get_hour_day_ranking(&week.friday, hour_points);
    hour_total_ranking += get_hour_day_ranking(&week.saturday, hour_points);

    hour_total_ranking
}

fn get_hour_day_ranking(day: &HashMap<u8, HourInfo>, hour_points: f32) -> f32 {
    let mut dead_hours_count = 0.;
    let mut dead_hour_buffer = 0.;
    let mut started = false;
    let mut space = true;
    for i in 0..=24 {
        if day.contains_key(&i) {
            if space && started {
                dead_hours_count += dead_hour_buffer;
                dead_hour_buffer = 0.;
            }

            started = true; 
            space = false;

            continue;
        } 

        if started {
            dead_hour_buffer += 1.;
        }
        space = true;
    }

    dead_hours_count * hour_points * -1.
}

fn generate_plans_recursive(valid_weeks: &mut Vec<Week>, current_class_index: usize, class_list: &Vec<Class>, current_week: Week) {
    if current_class_index >= class_list.len() {
        valid_weeks.push(current_week.clone());
        return
    }

    let current_class = &class_list[current_class_index];
    let current_class_schedules = &current_class.schedules;


    for schedule in current_class_schedules {
        let mut current_week_to_insert = current_week.clone();
        let result = current_week_to_insert.insert_schedule(&schedule, &current_class.class_name);

        if let Ok(_) = result {
            generate_plans_recursive(valid_weeks, current_class_index + 1, class_list, current_week_to_insert)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatedWeek {
    week: Week,
    puntuation: f32
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                domain::Day::Monday => Week::insert_block_hashmap(&mut self.monday, block, &schedule.schedule_name, class_name),
                domain::Day::Tuesday => Week::insert_block_hashmap(&mut self.tuesday, block, &schedule.schedule_name, class_name),
                domain::Day::Wednesday => Week::insert_block_hashmap(&mut self.wednesday, block, &schedule.schedule_name, class_name),
                domain::Day::Thursday => Week::insert_block_hashmap(&mut self.thursday, block, &schedule.schedule_name, class_name),
                domain::Day::Friday => Week::insert_block_hashmap(&mut self.friday, block, &schedule.schedule_name, class_name),
                domain::Day::Saturday => Week::insert_block_hashmap(&mut self.saturday, block, &schedule.schedule_name, class_name),
            }
        }

        Ok(())
    }

    fn insert_block_hashmap(day_map: &mut HashMap<u8, HourInfo>, block: &Block, schedule_name: &str, class_name: &str) {
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use domain::{Block, Class, Day, Schedule};

    use crate::{generate_plans_recursive, get_hour_day_ranking, HourInfo, Week};

    #[test]
    fn dead_hour_day_test() {

        let mut day = HashMap::new();

        day.insert(7, HourInfo {
            class_name: "idk".to_string(),
            schedule_name: "idkkk".to_string()
        });

        let mut points = get_hour_day_ranking(&day, 1.);

        assert_eq!(points, 0.);

        day.insert(9, HourInfo {
            class_name: "idk".to_string(),
            schedule_name: "idkkk".to_string()
        });

        points = get_hour_day_ranking(&day, 1.);

        assert_eq!(points, -1.);

    }

    #[test]
    fn week_recursive_test() {

        let classes = vec![
            Class {
                class_id: 0,
                class_name: "Math".to_string(),
                schedules: vec![
                    Schedule {
                        schedule_id: 0,
                        schedule_name: "Math 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 0,
                                start_hour: 7,
                                finish_hour: 9,
                                day: Day::Monday
                            }
                        ]
                    },
                    Schedule {
                        schedule_id: 1,
                        schedule_name: "Math 2".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 1,
                                start_hour: 7,
                                finish_hour: 9,
                                day: Day::Tuesday
                            }
                        ]
                    }
                ]
            },

            Class {
                class_id: 0,
                class_name: "Prog".to_string(),
                schedules: vec![
                    Schedule {
                        schedule_id: 2,
                        schedule_name: "Prog 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 1,
                                start_hour: 8,
                                finish_hour: 10,
                                day: Day::Saturday,
                            },
                            Block {
                                block_id: 2,
                                start_hour: 8,
                                finish_hour: 10,
                                day: Day::Monday
                            }
                        ]
                    },
                    Schedule {
                        schedule_id: 2,
                        schedule_name: "Prog 2".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 2,
                                start_hour: 10,
                                finish_hour: 12,
                                day: Day::Monday
                            },
                            Block {
                                block_id: 2,
                                start_hour: 12,
                                finish_hour: 14,
                                day: Day::Tuesday
                            }
                        ]
                    }
                ]
            }
        ];

        let mut valid_weeks: Vec<Week> = Vec::new();

        generate_plans_recursive(&mut valid_weeks, 0, &classes, Week::new());

        assert_eq!(valid_weeks.len(), 3);
    }

    #[test]
    fn week_recursive_test_no_options() {
        let classes = vec![
            Class {
                class_id: 1,
                class_name: "Math".to_string(),
                schedules: vec![
                    Schedule {
                        schedule_id: 1,
                        schedule_name: "Math 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 1,
                                start_hour: 7,
                                finish_hour: 9,
                                day: Day::Monday
                            }
                        ]
                    },
                    Schedule {
                        schedule_id: 2,
                        schedule_name: "Math 2".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 2,
                                start_hour: 7,
                                finish_hour: 9,
                                day: Day::Tuesday
                            }
                        ]
                    }
                ]
            },
            Class {
                class_id: 2,
                class_name: "Prog".to_string(),
                schedules: vec![
                    Schedule {
                        schedule_id: 3,
                        schedule_name: "Prog 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 3,
                                start_hour: 8,
                                finish_hour: 10,
                                day: Day::Saturday,
                            },
                            Block {
                                block_id: 4,
                                start_hour: 8,
                                finish_hour: 10,
                                day: Day::Monday
                            }
                        ]
                    },
                    Schedule {
                        schedule_id: 4,
                        schedule_name: "Prog 2".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 5,
                                start_hour: 10,
                                finish_hour: 12,
                                day: Day::Monday
                            },
                            Block {
                                block_id: 6,
                                start_hour: 12,
                                finish_hour: 14,
                                day: Day::Tuesday
                            }
                        ]
                    }
                ]
            },
            Class {
                class_id: 3,
                class_name: "Core".to_string(),
                schedules: vec![
                    Schedule {
                        schedule_id: 5,
                        schedule_name: "Core 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 7,
                                start_hour: 7,
                                finish_hour: 11,
                                day: Day::Monday
                            }
                        ]
                    }
                ]
            }
        ];
    }
}
