use sqlx::{SqlitePool, query, query_as};
use uuid::Uuid;
use domain::{Day, Block};

use super::{Schedule, Class};
use crate::{err::{self, RepositoryError, Result}, PlannerRepository};

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

use sqlx::{Pool, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::domain::{Class, Schedule, Block, Day};
use crate::err::{Result, RepositoryError};
use crate::repository::PlannerRepository;

pub struct SqlitePlannerRepository {
    pool: Pool<Sqlite>,
}

impl SqlitePlannerRepository {
    pub async fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlannerRepository for SqlitePlannerRepository {
    // Class Operations
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

    async fn delete_class(&self, user_id: i32, class_name: String) -> Result<()> {
        sqlx::query!(
            "DELETE FROM classes WHERE class_name = ? AND user_id = ?",
            class_name,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_class(&self, user_id: i32, class_name: String) -> Result<Class> {
        // First, retrieve the class
        let class_row = sqlx::query!(
            "SELECT class_name FROM classes WHERE class_name = ? AND user_id = ?",
            class_name,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Then, retrieve its schedules
        let schedule_rows = sqlx::query!(
            "SELECT DISTINCT schedule_id FROM schedule WHERE class_name = ?",
            class_name
        )
        .fetch_all(&self.pool)
        .await?;

        let mut schedules = Vec::new();
        for schedule_row in schedule_rows {
            let blocks_rows = sqlx::query!(
                "SELECT block_id, start_hour, finish_hour, day 
                 FROM block 
                 WHERE schedule_id = ?",
                schedule_row.schedule_id
            )
            .fetch_all(&self.pool)
            .await?;

            let blocks = blocks_rows.into_iter().map(|block_row| Block {
                start_hour: block_row.start_hour as u8,
                finish_hour: block_row.finish_hour as u8,
                day: Day::from(block_row.day),
            }).collect();

            schedules.push(Schedule { blocks });
        }

        Ok(Class {
            class_name: class_row.class_name,
            schedules,
        })
    }

    async fn get_classes(&self, user_id: i32) -> Result<Vec<Class>> {
        let class_rows = sqlx::query!(
            "SELECT class_name FROM classes WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut classes = Vec::new();
        for class_row in class_rows {
            let class = self.get_class(user_id, class_row.class_name).await?;
            classes.push(class);
        }

        Ok(classes)
    }

    // Schedule Operations
    async fn add_schedule(&self, class_name: String, schedule: Schedule) -> Result<()> {
        // Generate a unique schedule ID
        let schedule_id = Uuid::new_v4().to_string();

        for block in schedule.blocks {
            sqlx::query!(
                "INSERT INTO schedule (schedule_id, class_name) VALUES (?, ?)",
                schedule_id,
                class_name
            )
            .execute(&self.pool)
            .await?;

            sqlx::query!(
                "INSERT INTO block (start_hour, finish_hour, day, schedule_id) 
                 VALUES (?, ?, ?, ?)",
                block.start_hour as i32,
                block.finish_hour as i32,
                block.day.to_string(),
                schedule_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn delete_schedule(&self, class_name: String, schedule_id: String) -> Result<()> {
        // First, delete related blocks
        sqlx::query!(
            "DELETE FROM block WHERE schedule_id = ?",
            schedule_id
        )
        .execute(&self.pool)
        .await?;

        // Then, delete the schedule
        sqlx::query!(
            "DELETE FROM schedule WHERE schedule_id = ? AND class_name = ?",
            schedule_id,
            class_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Block Operations
    async fn add_block(&self, schedule_id: String, block: Block) -> Result<()> {
        sqlx::query!(
            "INSERT INTO block (start_hour, finish_hour, day, schedule_id) 
             VALUES (?, ?, ?, ?)",
            block.start_hour as i32,
            block.finish_hour as i32,
            block.day.to_string(),
            schedule_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_block(&self, block_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM block WHERE block_id = ?",
            block_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_blocks(&self, schedule_id: String) -> Result<Vec<Block>> {
        let blocks_rows = sqlx::query!(
            "SELECT block_id, start_hour, finish_hour, day 
             FROM block 
             WHERE schedule_id = ?",
            schedule_id
        )
        .fetch_all(&self.pool)
        .await?;

        let blocks = blocks_rows.into_iter().map(|block_row| Block {
            start_hour: block_row.start_hour as u8,
            finish_hour: block_row.finish_hour as u8,
            day: Day::from(block_row.day),
        }).collect();

        Ok(blocks)
    }
}
