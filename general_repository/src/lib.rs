mod err;
pub mod postgres_db;

use domain::{Block, BlockInfo, Class, Course, CourseSummary, Schedule};
use mockall::automock;

#[automock]
pub trait PlannerRepository: Send {
    // Class operations
    async fn add_class(&self, user_id: i32, class_name: String, course_id: i32) -> err::Result<()>;
    async fn delete_class(&self, user_id: i32, class_id: i32) -> err::Result<()>;
    async fn get_class(&self, user_id: i32, class_id: i32) -> err::Result<Class>;
    async fn get_classes(&self, user_id: i32) -> err::Result<Vec<Class>>;
    async fn get_classes_id(&self, user_id: i32) -> err::Result<Vec<i32>>;
    async fn set_class_course(&self, user_id: i32, class_id: i32, course_id: i32) -> err::Result<()>;

    /// Wipes every course (and, via cascade, every class/schedule/block) for all users.
    async fn delete_all_classes(&self) -> err::Result<()>;

    // Course operations (shared catalog)
    async fn add_course(&self, course_name: String) -> err::Result<i32>;
    async fn search_courses(&self, query: String) -> err::Result<Vec<CourseSummary>>;
    async fn get_course(&self, course_id: i32) -> err::Result<Course>;

    // Schedule operations (require course_id)
    async fn add_schedule(&self, course_id: i32, schedule_name: String) -> err::Result<()>;
    async fn delete_schedule(&self, schedule_id: i32) -> err::Result<()>;
    async fn get_schedules(&self, course_id: i32) -> err::Result<Vec<Schedule>>;

    // Block operations
    async fn add_block(&self, schedule_id: i32, block: BlockInfo) -> err::Result<()>;
    async fn delete_block(&self, block_id: i32) -> err::Result<()>;
    async fn get_blocks(&self, schedule_id: i32) -> err::Result<Vec<Block>>;

    async fn add_user(&self) -> err::Result<i32>;
}

