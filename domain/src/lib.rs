use serde::{Deserialize, Serialize};

// This is the domain but in the example branch

// This is a bad comment

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<String> for Day {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Monday" => Day::Monday,
            "Tuesday" => Day::Tuesday,
            "Wednesday" => Day::Wednesday,
            "Thursday" => Day::Thursday,
            "Friday" => Day::Friday,
            "Saturday" => Day::Saturday,
            _ => panic!("Invalid day string: {}", value),
        }
    }
}

impl ToString for Day {
    fn to_string(&self) -> String {
        match self {
            Day::Monday => "Monday".to_string(),
            Day::Tuesday => "Tuesday".to_string(),
            Day::Wednesday => "Wednesday".to_string(),
            Day::Thursday => "Thursday".to_string(),
            Day::Friday => "Friday".to_string(),
            Day::Saturday => "Saturday".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RankedParametersEndpoint {
    pub ranked_parameters: RankingParameters,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRankingInput {
    pub user_id: i32,
    pub ranking_parameters: RankingParameters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupPlanningRequest {
    pub users: Vec<UserRankingInput>,
    pub match_weight: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub start_hour: u8,
    pub finish_hour: u8,
    pub day: Day,
    pub block_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RankingParameters {
    pub cost_hour: f32,
    pub cost_day: f32,
    pub exit_time_multiplier: f32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockInfo {
    pub start_hour: u8,
    pub finish_hour: u8,
    pub day: Day,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateBlock {
    pub block: BlockInfo,
    pub schedule_id: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    pub blocks: Vec<Block>,
    pub schedule_name: String,
    pub schedule_id: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Class {
    pub class_id: i32,
    pub class_name: String,
    pub course_id: i32,
    pub schedules: Vec<Schedule>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSchedule {
    pub course_id: i32,
    pub schedule_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateClass {
    pub user_id: i32,
    pub class_name: String,
    /// `None` creates a brand new (shared) course named `class_name`.
    /// `Some(course_id)` links this class to an existing shared course.
    #[serde(default)]
    pub course_id: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    pub course_id: i32,
    pub course_name: String,
    pub schedules: Vec<Schedule>,
    /// How many classes (across all users) currently link to this course.
    pub class_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CourseSummary {
    pub course_id: i32,
    pub course_name: String,
    /// How many classes (across all users) currently link to this course.
    pub class_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteClass {
    pub user_id: i32,
    pub class_id: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteAllClasses {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelinkClass {
    pub user_id: i32,
    pub class_id: i32,
    /// `Some(course_id)` joins this class to an existing shared course.
    /// `None` detaches it into a brand new (personal) course of the same name.
    #[serde(default)]
    pub course_id: Option<i32>,
}
