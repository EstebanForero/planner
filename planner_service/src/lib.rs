mod err;
use std::{collections::{HashMap, HashSet}, usize};

use domain::{Block, BlockInfo, Class, Course, CourseSummary, GroupPlanningRequest, RankingParameters, Schedule};
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

    pub async fn create_schedule(&self, course_id: i32, schedule_name: String) -> err::Result<()> {
        self.repository.add_schedule(course_id, schedule_name).await.map_err(|_| {
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

    pub async fn create_class(&self, user_id: i32, class_name: String, course_id: Option<i32>) -> err::Result<()> {
        let course_id = match course_id {
            Some(course_id) => course_id,
            None => self.repository.add_course(class_name.clone()).await.map_err(|_| {
                error!("add course has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

                PlannerError::AddCourseError
            })?,
        };

        self.repository.add_class(user_id, class_name, course_id).await.map_err(|_| {
            error!("add class has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::AddClassError
        })?;

        Ok(())
    }

    pub async fn search_courses(&self, query: String) -> err::Result<Vec<CourseSummary>> {
        let courses = self.repository.search_courses(query).await.map_err(|_| {
            error!("search courses has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::SearchCoursesError
        })?;

        Ok(courses)
    }

    pub async fn obtain_course(&self, course_id: i32) -> err::Result<Course> {
        let course = self.repository.get_course(course_id).await.map_err(|_| {
            error!("get course has en error pls solve ahhhhhhhhhhhhhhhhhh ahhhhhhhhhhhhhhhhhhhh it hurts");

            PlannerError::GetCourseError
        })?;

        Ok(course)
    }

    pub async fn remove_class(&self, user_id: i32, class_id: i32) -> err::Result<()> {
        self.repository.delete_class(user_id, class_id).await.map_err(|err| {
            error!("remove class has en error: {}", err);

            PlannerError::RemoveClassError
        })?;

        Ok(())
    }

    /// Re-link a class to a different course. `Some(course_id)` joins an existing
    /// (possibly shared) course; `None` detaches the class into a brand new course
    /// of the same name, making it personal again.
    pub async fn relink_class(&self, user_id: i32, class_id: i32, course_id: Option<i32>) -> err::Result<()> {
        let target_course_id = match course_id {
            Some(course_id) => course_id,
            None => {
                let class = self.repository.get_class(user_id, class_id).await.map_err(|_| {
                    PlannerError::GetClassError
                })?;

                self.repository.add_course(class.class_name).await.map_err(|_| {
                    PlannerError::AddCourseError
                })?
            }
        };

        self.repository.set_class_course(user_id, class_id, target_course_id).await.map_err(|_| {
            PlannerError::RelinkClassError
        })?;

        Ok(())
    }

    /// Wipes every course/class/schedule/block for all users. Caller is responsible
    /// for restricting who is allowed to invoke this.
    pub async fn delete_all_classes(&self) -> err::Result<()> {
        self.repository.delete_all_classes().await.map_err(|_| {
            PlannerError::DeleteAllClassesError
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

        Ok(rank_weeks_for_classes(&classes, &ranking_parameters))
    }

    pub async fn rank_group_plannings(&self, request: GroupPlanningRequest) -> err::Result<Vec<GroupRatedWeek>> {
        let mut user_ids = Vec::new();
        let mut user_classes = Vec::new();
        let mut ranking_params = Vec::new();

        for user_input in &request.users {
            let classes = self.obtain_classes(user_input.user_id).await?;

            user_ids.push(user_input.user_id);
            user_classes.push(classes);
            ranking_params.push(user_input.ranking_parameters.clone());
        }

        let shared_courses = find_shared_courses(&user_classes);

        let user_candidates: Vec<Vec<RatedWeek>> = user_classes.iter().zip(ranking_params.iter())
            .map(|(classes, params)| {
                let ranked = rank_weeks_for_classes(classes, params);
                let groups = group_by_signature(ranked, &shared_courses);

                flatten_candidates(groups)
            })
            .collect();

        let mut combos = Vec::new();
        cross_product_recursive(&user_candidates, 0, &mut Vec::new(), &mut combos);

        let mut group_weeks: Vec<GroupRatedWeek> = combos.into_iter().map(|combo| {
            let weeks: Vec<&Week> = combo.iter().map(|rated_week| &rated_week.week).collect();
            let (match_score, course_agreements) = compute_match_score(&weeks, &shared_courses);

            let individual_score: f32 = combo.iter().map(|rated_week| rated_week.puntuation).sum();
            let total_score = individual_score + request.match_weight * match_score;

            let user_weeks = user_ids.iter().zip(combo).map(|(&user_id, rated_week)| UserRatedWeek {
                user_id,
                rated_week,
            }).collect();

            GroupRatedWeek {
                user_weeks,
                match_score,
                course_agreements,
                total_score,
            }
        }).collect();

        group_weeks.sort_unstable_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
        group_weeks.truncate(MAX_GROUP_RESULTS);

        Ok(group_weeks)
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

fn get_exit_time_ranking(week: &Week, exit_time_multiplier: f32) -> f32 {
    let mut exit_time_ranking = 0.;

    exit_time_ranking += get_exit_time_ranking_day(&week.monday);
    exit_time_ranking += get_exit_time_ranking_day(&week.tuesday);
    exit_time_ranking += get_exit_time_ranking_day(&week.wednesday);
    exit_time_ranking += get_exit_time_ranking_day(&week.thursday);
    exit_time_ranking += get_exit_time_ranking_day(&week.friday);
    exit_time_ranking += get_exit_time_ranking_day(&week.saturday);

    exit_time_ranking * exit_time_multiplier * -1.
}

fn get_exit_time_ranking_day(day: &HashMap<u8, HourInfo>) -> f32 {

    if day.is_empty() {
        return 0.
    }

    for time in (0..=24).rev() {
        if day.contains_key(&time) {
            return exit_time_function(time)
        }
    }

    0.
}

fn exit_time_function(hour: u8) -> f32 {
    return ((hour as f32).powf(1.7)) / 200.;
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

fn generate_plans_recursive(valid_weeks: &mut Vec<Week>, current_class_index: usize, class_list: &[Class], current_week: Week) {
    if current_class_index >= class_list.len() {
        valid_weeks.push(current_week.clone());
        return
    }

    let current_class = &class_list[current_class_index];
    let current_class_schedules = &current_class.schedules;


    for schedule in current_class_schedules {
        let mut current_week_to_insert = current_week.clone();
        let result = current_week_to_insert.insert_schedule(&schedule, &current_class.class_name, current_class.course_id);

        if let Ok(_) = result {
            generate_plans_recursive(valid_weeks, current_class_index + 1, class_list, current_week_to_insert)
        }
    }
}

fn rank_weeks_for_classes(classes: &[Class], params: &RankingParameters) -> Vec<RatedWeek> {
    let mut valid_weeks = Vec::new();
    generate_plans_recursive(&mut valid_weeks, 0, classes, Week::new());

    let mut ranked: Vec<RatedWeek> = valid_weeks.into_iter().map(|week| {
        let puntuation = get_week_ranking(&week, params.cost_day)
            + get_hour_ranking(&week, params.cost_hour)
            + get_exit_time_ranking(&week, params.exit_time_multiplier);

        RatedWeek { week, puntuation }
    }).collect();

    ranked.sort_unstable_by_key(|rw| (rw.puntuation * 100.) as i32);
    ranked.into_iter().rev().collect()
}

/// Tunables for the group-planning combination search. `MAX_SIGNATURES_PER_USER`
/// signature combos raised to the number of users stays small for the realistic
/// group sizes (N <= 4) this feature targets.
const TOP_K_PER_SIGNATURE: usize = 3;
const MAX_SIGNATURES_PER_USER: usize = 12;
const MAX_GROUP_RESULTS: usize = 20;

/// course_ids that appear in at least two users' class lists.
fn find_shared_courses(user_classes: &[Vec<Class>]) -> Vec<i32> {
    let mut course_user_counts: HashMap<i32, u32> = HashMap::new();

    for classes in user_classes {
        let course_ids: HashSet<i32> = classes.iter().map(|class| class.course_id).collect();

        for course_id in course_ids {
            *course_user_counts.entry(course_id).or_insert(0) += 1;
        }
    }

    course_user_counts.into_iter()
        .filter(|&(_, count)| count >= 2)
        .map(|(course_id, _)| course_id)
        .collect()
}

/// A week's choices for the shared courses, as (course_id, schedule_id) pairs.
/// Two weeks with the same signature agree on every shared course.
fn week_signature(week: &Week, shared_courses: &[i32]) -> Vec<(i32, i32)> {
    let mut signature: Vec<(i32, i32)> = shared_courses.iter()
        .filter_map(|&course_id| week.selections.get(&course_id).map(|&schedule_id| (course_id, schedule_id)))
        .collect();

    signature.sort_unstable();
    signature
}

/// Groups `ranked` (already sorted best-first) by signature, keeping the top
/// `TOP_K_PER_SIGNATURE` weeks per group and the top `MAX_SIGNATURES_PER_USER`
/// groups (by best score, which is preserved by input order).
fn group_by_signature(ranked: Vec<RatedWeek>, shared_courses: &[i32]) -> Vec<Vec<RatedWeek>> {
    let mut groups: Vec<(Vec<(i32, i32)>, Vec<RatedWeek>)> = Vec::new();

    for rated_week in ranked {
        let signature = week_signature(&rated_week.week, shared_courses);

        match groups.iter_mut().find(|(sig, _)| *sig == signature) {
            Some((_, weeks)) if weeks.len() < TOP_K_PER_SIGNATURE => weeks.push(rated_week),
            Some(_) => {}
            None => groups.push((signature, vec![rated_week])),
        }
    }

    groups.truncate(MAX_SIGNATURES_PER_USER);
    groups.into_iter().map(|(_, weeks)| weeks).collect()
}

fn flatten_candidates(groups: Vec<Vec<RatedWeek>>) -> Vec<RatedWeek> {
    groups.into_iter().flatten().collect()
}

fn cross_product_recursive(user_candidates: &[Vec<RatedWeek>], current_user_index: usize, current_combo: &mut Vec<RatedWeek>, combos: &mut Vec<Vec<RatedWeek>>) {
    if current_user_index >= user_candidates.len() {
        combos.push(current_combo.clone());
        return
    }

    for candidate in &user_candidates[current_user_index] {
        current_combo.push(candidate.clone());
        cross_product_recursive(user_candidates, current_user_index + 1, current_combo, combos);
        current_combo.pop();
    }
}

/// For each shared course, scores how often the group's weeks agree on the
/// same schedule_id (1.0 = everyone picked the same section).
fn compute_match_score(weeks: &[&Week], shared_courses: &[i32]) -> (f32, Vec<CourseAgreement>) {
    let n = weeks.len() as u32;
    let total_pairs = n * n.saturating_sub(1) / 2;

    let mut match_score = 0.;
    let mut agreements = Vec::new();

    for &course_id in shared_courses {
        let mut buckets: HashMap<i32, u32> = HashMap::new();

        for week in weeks {
            if let Some(&schedule_id) = week.selections.get(&course_id) {
                *buckets.entry(schedule_id).or_insert(0) += 1;
            }
        }

        let agreement_pairs: u32 = buckets.values().map(|&count| count * count.saturating_sub(1) / 2).sum();
        let agreement_score = if total_pairs == 0 { 0. } else { agreement_pairs as f32 / total_pairs as f32 };

        match_score += agreement_score;
        agreements.push(CourseAgreement { course_id, agreement_pairs, total_pairs, agreement_score });
    }

    (match_score, agreements)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatedWeek {
    week: Week,
    puntuation: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseAgreement {
    pub course_id: i32,
    pub agreement_pairs: u32,
    pub total_pairs: u32,
    pub agreement_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRatedWeek {
    pub user_id: i32,
    pub rated_week: RatedWeek,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRatedWeek {
    pub user_weeks: Vec<UserRatedWeek>,
    pub match_score: f32,
    pub course_agreements: Vec<CourseAgreement>,
    pub total_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Week {
    monday: HashMap<u8, HourInfo>,
    tuesday: HashMap<u8, HourInfo>,
    wednesday: HashMap<u8, HourInfo>,
    thursday: HashMap<u8, HourInfo>,
    friday: HashMap<u8, HourInfo>,
    saturday: HashMap<u8, HourInfo>,
    /// course_id -> schedule_id chosen for that course in this week.
    selections: HashMap<i32, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HourInfo {
    class_name: String,
    schedule_name: String,
    course_id: i32
}

impl Week {
    pub fn new() -> Self {
        Self {
            monday: HashMap::new(),
            tuesday: HashMap::new(),
            wednesday: HashMap::new(),
            thursday: HashMap::new(),
            friday: HashMap::new(),
            saturday: HashMap::new(),
            selections: HashMap::new()
        }
    }

    pub fn insert_schedule(&mut self, schedule: &Schedule, class_name: &str, course_id: i32) -> err::Result<()> {
        for block in &schedule.blocks {
            if self.is_collition(block) {
                return Err(err::PlannerError::AddScheduleError)
            }
        }

        for block in &schedule.blocks {
            match block.day {
                domain::Day::Monday => Week::insert_block_hashmap(&mut self.monday, block, &schedule.schedule_name, class_name, course_id),
                domain::Day::Tuesday => Week::insert_block_hashmap(&mut self.tuesday, block, &schedule.schedule_name, class_name, course_id),
                domain::Day::Wednesday => Week::insert_block_hashmap(&mut self.wednesday, block, &schedule.schedule_name, class_name, course_id),
                domain::Day::Thursday => Week::insert_block_hashmap(&mut self.thursday, block, &schedule.schedule_name, class_name, course_id),
                domain::Day::Friday => Week::insert_block_hashmap(&mut self.friday, block, &schedule.schedule_name, class_name, course_id),
                domain::Day::Saturday => Week::insert_block_hashmap(&mut self.saturday, block, &schedule.schedule_name, class_name, course_id),
            }
        }

        self.selections.insert(course_id, schedule.schedule_id);

        Ok(())
    }

    fn insert_block_hashmap(day_map: &mut HashMap<u8, HourInfo>, block: &Block, schedule_name: &str, class_name: &str, course_id: i32) {
        for hour in block.start_hour..block.finish_hour {
            day_map.insert(hour, HourInfo {
                schedule_name: schedule_name.to_string(),
                class_name: class_name.to_string(),
                course_id
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

    use domain::{Block, Class, Day, RankingParameters, Schedule};

    use crate::{compute_match_score, cross_product_recursive, find_shared_courses, flatten_candidates, generate_plans_recursive, get_hour_day_ranking, group_by_signature, rank_weeks_for_classes, HourInfo, Week};

    #[test]
    fn dead_hour_day_test() {

        let mut day = HashMap::new();

        day.insert(7, HourInfo {
            class_name: "idk".to_string(),
            schedule_name: "idkkk".to_string(),
            course_id: 1
        });

        let mut points = get_hour_day_ranking(&day, 1.);

        assert_eq!(points, 0.);

        day.insert(9, HourInfo {
            class_name: "idk".to_string(),
            schedule_name: "idkkk".to_string(),
            course_id: 1
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
                course_id: 100,
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
                course_id: 200,
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
                course_id: 1,
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
                course_id: 2,
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
                course_id: 3,
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

    #[test]
    fn selections_populated_test() {
        let classes = vec![
            Class {
                class_id: 1,
                class_name: "Math".to_string(),
                course_id: 100,
                schedules: vec![
                    Schedule {
                        schedule_id: 10,
                        schedule_name: "Math 1".to_string(),
                        blocks: vec![
                            Block {
                                block_id: 1,
                                start_hour: 7,
                                finish_hour: 9,
                                day: Day::Monday
                            }
                        ]
                    }
                ]
            }
        ];

        let mut valid_weeks: Vec<Week> = Vec::new();
        generate_plans_recursive(&mut valid_weeks, 0, &classes, Week::new());

        assert_eq!(valid_weeks.len(), 1);
        assert_eq!(valid_weeks[0].selections.get(&100), Some(&10));
    }

    #[test]
    fn compute_match_score_full_agreement_test() {
        let mut week_a = Week::new();
        week_a.selections.insert(100, 1);

        let mut week_b = Week::new();
        week_b.selections.insert(100, 1);

        let (match_score, agreements) = compute_match_score(&[&week_a, &week_b], &[100]);

        assert_eq!(match_score, 1.0);
        assert_eq!(agreements.len(), 1);
        assert_eq!(agreements[0].course_id, 100);
        assert_eq!(agreements[0].agreement_pairs, 1);
        assert_eq!(agreements[0].total_pairs, 1);
        assert_eq!(agreements[0].agreement_score, 1.0);
    }

    #[test]
    fn compute_match_score_partial_agreement_test() {
        let mut week_a = Week::new();
        week_a.selections.insert(100, 1);

        let mut week_b = Week::new();
        week_b.selections.insert(100, 1);

        let mut week_c = Week::new();
        week_c.selections.insert(100, 2);

        let (match_score, agreements) = compute_match_score(&[&week_a, &week_b, &week_c], &[100]);

        // 3 users -> 3 pairs total, only (A, B) agree -> 1/3
        assert_eq!(agreements[0].total_pairs, 3);
        assert_eq!(agreements[0].agreement_pairs, 1);
        assert!((agreements[0].agreement_score - (1.0 / 3.0)).abs() < 1e-6);
        assert!((match_score - (1.0 / 3.0)).abs() < 1e-6);
    }

    #[test]
    fn compute_match_score_no_shared_courses_test() {
        let week_a = Week::new();
        let week_b = Week::new();

        let (match_score, agreements) = compute_match_score(&[&week_a, &week_b], &[]);

        assert_eq!(match_score, 0.0);
        assert!(agreements.is_empty());
    }

    #[test]
    fn group_planning_prefers_aligned_schedules_test() {
        let shared_schedules = vec![
            Schedule {
                schedule_id: 1,
                schedule_name: "Section A".to_string(),
                blocks: vec![Block { block_id: 1, start_hour: 7, finish_hour: 9, day: Day::Monday }]
            },
            Schedule {
                schedule_id: 2,
                schedule_name: "Section B".to_string(),
                blocks: vec![Block { block_id: 2, start_hour: 7, finish_hour: 9, day: Day::Tuesday }]
            },
        ];

        let user_a_classes = vec![
            Class { class_id: 1, class_name: "Shared Course".to_string(), course_id: 100, schedules: shared_schedules.clone() },
            Class {
                class_id: 2, class_name: "A Only".to_string(), course_id: 101,
                schedules: vec![Schedule {
                    schedule_id: 3, schedule_name: "A Only 1".to_string(),
                    blocks: vec![Block { block_id: 3, start_hour: 7, finish_hour: 9, day: Day::Thursday }]
                }]
            },
        ];

        let user_b_classes = vec![
            Class { class_id: 3, class_name: "Shared Course".to_string(), course_id: 100, schedules: shared_schedules.clone() },
            Class {
                class_id: 4, class_name: "B Only".to_string(), course_id: 102,
                schedules: vec![Schedule {
                    schedule_id: 4, schedule_name: "B Only 1".to_string(),
                    blocks: vec![Block { block_id: 4, start_hour: 7, finish_hour: 9, day: Day::Friday }]
                }]
            },
        ];

        let params = RankingParameters { cost_hour: 1., cost_day: 1., exit_time_multiplier: 1. };

        let ranked_a = rank_weeks_for_classes(&user_a_classes, &params);
        let ranked_b = rank_weeks_for_classes(&user_b_classes, &params);

        // Both schedule choices for the shared course are equally "good" for each user.
        assert_eq!(ranked_a.len(), 2);
        assert_eq!(ranked_b.len(), 2);
        assert!((ranked_a[0].puntuation - ranked_a[1].puntuation).abs() < 1e-6);
        assert!((ranked_b[0].puntuation - ranked_b[1].puntuation).abs() < 1e-6);

        let shared_courses = find_shared_courses(&[user_a_classes, user_b_classes]);
        assert_eq!(shared_courses, vec![100]);

        let groups_a = group_by_signature(ranked_a, &shared_courses);
        let groups_b = group_by_signature(ranked_b, &shared_courses);

        assert_eq!(groups_a.len(), 2);
        assert_eq!(groups_b.len(), 2);

        let candidates_a = flatten_candidates(groups_a);
        let candidates_b = flatten_candidates(groups_b);

        let mut combos = Vec::new();
        cross_product_recursive(&[candidates_a, candidates_b], 0, &mut Vec::new(), &mut combos);

        assert_eq!(combos.len(), 4);

        let match_weight = 10.0;

        let mut scored: Vec<(f32, f32)> = combos.into_iter().map(|combo| {
            let weeks: Vec<&Week> = combo.iter().map(|rated_week| &rated_week.week).collect();
            let (match_score, _) = compute_match_score(&weeks, &shared_courses);
            let individual: f32 = combo.iter().map(|rated_week| rated_week.puntuation).sum();

            (individual + match_weight * match_score, match_score)
        }).collect();

        scored.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Combos where both users land on the same shared schedule_id (match_score == 1.0)
        // must outrank combos where they diverge (match_score == 0.0).
        assert_eq!(scored[0].1, 1.0);
        assert_eq!(scored[1].1, 1.0);
        assert_eq!(scored[2].1, 0.0);
        assert_eq!(scored[3].1, 0.0);
    }
}
