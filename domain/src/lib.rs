
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
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
            Day::Sunday => "Sunday".to_string(),
        }
    }
}

struct Block {
    start_hour: u8,
    finish_hour: u8,
    day: Day
}

struct Schedule {
    blocks: Vec<Block>
}

struct Class {
    class_name: String,
    schedules: Vec<Schedule>
}
