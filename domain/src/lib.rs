use serde::{Deserialize, Serialize};


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
    pub user_id: i32
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
    pub cost_day: f32
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
    pub schedules: Vec<Schedule>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSchedule {
    pub class_id: i32,
    pub schedule_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateClass {
    pub user_id: i32,
    pub class_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteClass {
    pub user_id: i32,
    pub class_id: i32
}
