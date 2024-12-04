use sqlx::PgPool;
use domain::{Block, BlockInfo, Day};

use super::{Schedule, Class};
use crate::{err::{self, RepositoryError, Result}, PlannerRepository};

#[derive(Clone)]
pub struct PostgresPlannerRepository {
    pool: PgPool,
}

impl PostgresPlannerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn generate_pool(db_url: &str) -> Self {
        match PgPool::connect(db_url).await {
            Ok(pool) => {
                PostgresPlannerRepository { pool }
            },
            Err(_) => {
                panic!("Could not connect to database. Please ensure the database exists.");
            }
        }
    }
}

impl PlannerRepository for PostgresPlannerRepository {
    async fn add_class(&self, user_id: i32, class_name: String) -> Result<()> {
        sqlx::query!(
            "INSERT INTO classes (class_name, user_id) VALUES ($1, $2)",
            class_name,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_class(&self, user_id: i32, class_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM classes WHERE class_id = $1 AND user_id = $2",
            class_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::DatabaseError(sqlx::Error::RowNotFound));
        }

        Ok(())
    }

    async fn get_class(&self, user_id: i32, class_id: i32) -> Result<Class> {
        let class_row = sqlx::query!(
            "SELECT class_name, class_id FROM classes WHERE class_id = $1 AND user_id = $2",
            class_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let schedules = self.get_schedules(class_id).await?;

        Ok(Class {
            class_name: class_row.class_name,
            schedules,
            class_id: class_row.class_id
        })
    }

    async fn get_classes(&self, user_id: i32) -> Result<Vec<Class>> {
        let class_rows = sqlx::query!(
            "SELECT class_id, class_name FROM classes WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut classes = Vec::new();
        for class_row in class_rows {
            let schedules = self.get_schedules(class_row.class_id).await?;
            classes.push(Class {
                class_name: class_row.class_name,
                schedules,
                class_id: class_row.class_id
            });
        }

        Ok(classes)
    }

    async fn add_schedule(&self, class_id: i32, schedule_name: String) -> Result<()> {
        sqlx::query!(
            "INSERT INTO schedule (class_id, schedule_name) VALUES ($1, $2)",
            class_id,
            schedule_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_schedule(&self, schedule_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM schedule WHERE schedule_id = $1",
            schedule_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::DatabaseError(sqlx::Error::RowNotFound));
        }

        Ok(())
    }

    async fn get_schedules(&self, class_id: i32) -> Result<Vec<Schedule>> {
        let schedule_rows = sqlx::query!(
            "SELECT schedule_id, schedule_name FROM schedule WHERE class_id = $1",
            class_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut schedules = Vec::new();
        for schedule_row in schedule_rows {
            let blocks = self.get_blocks(schedule_row.schedule_id).await?;
            schedules.push(Schedule { 
                blocks, 
                schedule_name: schedule_row.schedule_name, 
                schedule_id: schedule_row.schedule_id 
            });
        }

        Ok(schedules)
    }

    async fn add_block(&self, schedule_id: i32, block: BlockInfo) -> Result<()> {
        let day_string = block.day.clone().to_string();
        sqlx::query!(
            "INSERT INTO block (start_hour, finish_hour, day, schedule_id) VALUES ($1, $2, $3, $4)",
            block.start_hour as i16,
            block.finish_hour as i16,
            day_string,
            schedule_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_block(&self, block_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM block WHERE block_id = $1",
            block_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::DatabaseError(sqlx::Error::RowNotFound));
        }

        Ok(())
    }

    async fn get_blocks(&self, schedule_id: i32) -> Result<Vec<Block>> {
        let block_rows = sqlx::query!(
            "SELECT start_hour, finish_hour, day, block_id FROM block WHERE schedule_id = $1",
            schedule_id
        )
        .fetch_all(&self.pool)
        .await?;

        let blocks = block_rows.into_iter().map(|row| Block {
            start_hour: row.start_hour as u8,
            finish_hour: row.finish_hour as u8,
            day: Day::from(row.day),
            block_id: row.block_id,
        }).collect();

        Ok(blocks)
    }

    async fn add_user(&self) -> err::Result<i32> {
        let result = sqlx::query!(
            "INSERT INTO planner_user DEFAULT VALUES RETURNING user_id"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.user_id)
    }
}
