use sqlx::{SqlitePool, query, query_as};
use uuid::Uuid;
use domain::{Day, Block};

use super::{Schedule, Class};
use crate::{err::{self, RepositoryError, Result}, PlannerRepository};

#[derive(Clone)]
pub struct SqlitePlannerRepository {
    pool: SqlitePool,
}

impl SqlitePlannerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn generate_pool() -> Self {
        let database_url = "sqlite://db/local.db";

        let pool = SqlitePool::connect(&database_url).await.expect("Errror creating database");

        SqlitePlannerRepository { pool }
    }
}

impl PlannerRepository for SqlitePlannerRepository {
    async fn add_class(&self, user_id: i32, class_name: String) -> Result<()> {
        sqlx::query!(
            "INSERT INTO classes (class_name, user_id) VALUES (?, ?)",
            class_name,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_class(&self, user_id: i32, class_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM classes WHERE class_id = ? AND user_id = ?",
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
            "SELECT class_name, class_id FROM classes WHERE class_id = ? AND user_id = ?",
            class_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let schedules = self.get_schedules(class_id).await?;

        Ok(Class {
            class_name: class_row.class_name,
            schedules,
            class_id: class_row.class_id as i32
        })
    }

    async fn get_classes(&self, user_id: i32) -> Result<Vec<Class>> {
        let class_rows = sqlx::query!(
            "SELECT class_id, class_name FROM classes WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut classes = Vec::new();
        for class_row in class_rows {
            let schedules = self.get_schedules(class_row.class_id as i32).await?;
            classes.push(Class {
                class_name: class_row.class_name,
                schedules,
                class_id: class_row.class_id as i32
            });
        }

        Ok(classes)
    }

    async fn add_schedule(&self, class_id: i32, schedule_name: String) -> Result<()> {
        sqlx::query!(
            "INSERT INTO schedule (class_id, schedule_name) VALUES (?, ?)",
            class_id,
            schedule_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_schedule(&self, schedule_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM schedule WHERE schedule_id = ?",
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
            "SELECT schedule_id, schedule_name FROM schedule WHERE class_id = ?",
            class_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut schedules = Vec::new();
        for schedule_row in schedule_rows {
            let blocks = self.get_blocks(schedule_row.schedule_id as i32).await?;
            schedules.push(Schedule { blocks, schedule_name: schedule_row.schedule_name });
        }

        Ok(schedules)
    }

    async fn add_block(&self, schedule_id: i32, block: Block) -> Result<()> {
        let day_string = block.day.clone().to_string();
        sqlx::query!(
            "INSERT INTO block (start_hour, finish_hour, day, schedule_id) VALUES (?, ?, ?, ?)",
            block.start_hour,
            block.finish_hour,
            day_string,
            schedule_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_block(&self, block_id: i32) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM block WHERE block_id = ?",
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
            "SELECT start_hour, finish_hour, day FROM block WHERE schedule_id = ?",
            schedule_id
        )
        .fetch_all(&self.pool)
        .await?;

        let blocks = block_rows.into_iter().map(|row| Block {
            start_hour: row.start_hour as u8,
            finish_hour: row.finish_hour as u8,
            day: Day::from(row.day)
        }).collect();

        Ok(blocks)
    }

    async fn add_user(&self) -> err::Result<i32> {
        let result = sqlx::query!(
        "INSERT INTO planner_user DEFAULT VALUES; SELECT last_insert_rowid() as user_id;"
    )
            .fetch_one(&self.pool)
        .await?;

        Ok(result.user_id as i32)
    }
}
