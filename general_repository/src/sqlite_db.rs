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

//impl PlannerRepository for SqlitePlannerRepository {
//    async fn add_schedule(
//        &self, 
//        user_id: Uuid, 
//        class_name: String, 
//        schedule: Schedule
//    ) -> Result<()> {
//        let schedule_id = Uuid::new_v4();
//
//        let mut tx = self.pool.begin().await?;
//
//        query!(
//            "INSERT INTO schedule (schedule_id, class_name) VALUES (?, ?)",
//            schedule_id,
//            class_name
//        )
//        .execute(&mut *tx)
//        .await?;
//
//        for block in schedule.blocks {
//            query!(
//                "INSERT INTO block (block_id, schedule_id, start_hour, finish_hour, day) VALUES (?, ?, ?, ?, ?)",
//                Uuid::new_v4(),
//                schedule_id,
//                block.start_hour as i64,
//                block.finish_hour as i64,
//                block.day.to_string()
//            )
//            .execute(&mut *tx)
//            .await?;
//        }
//
//        tx.commit().await?;
//        Ok(())
//    }
//
//    async fn delete_schedule(
//        &self, 
//        user_id: Uuid, 
//        class_name: String, 
//        schedule_id: Uuid
//    ) -> Result<()> {
//        let mut tx = self.pool.begin().await?;
//
//        query!("DELETE FROM block WHERE schedule_id = ?", schedule_id)
//            .execute(&mut *tx)
//            .await?;
//
//        query!("DELETE FROM schedule WHERE schedule_id = ? AND class_name = ?", schedule_id, class_name)
//            .execute(&mut *tx)
//            .await?;
//
//        tx.commit().await?;
//        Ok(())
//    }
//
//    async fn add_class(
//        &self, 
//        user_id: Uuid, 
//        class_name: String
//    ) -> Result<()> {
//        query!(
//            "INSERT INTO classes (user_id, class_name) VALUES (?, ?)",
//            user_id,
//            class_name
//        )
//        .execute(&self.pool)
//        .await?;
//
//        Ok(())
//    }
//
//    async fn delete_class(
//        &self, 
//        user_id: Uuid, 
//        class_name: String
//    ) -> Result<()> {
//        let mut tx = self.pool.begin().await?;
//
//        let schedules = query!(
//            "SELECT schedule_id FROM schedule WHERE class_name = ?", 
//            class_name
//        )
//        .fetch_all(&mut *tx)
//        .await?;
//
//        for schedule in schedules {
//            query!("DELETE FROM block WHERE schedule_id = ?", schedule.schedule_id)
//                .execute(&mut *tx)
//                .await?;
//        }
//
//        query!("DELETE FROM schedule WHERE class_name = ?", class_name)
//            .execute(&mut *tx)
//            .await?;
//
//        query!("DELETE FROM classes WHERE user_id = ? AND class_name = ?", user_id, class_name)
//            .execute(&mut *tx)
//            .await?;
//
//        tx.commit().await?;
//        Ok(())
//    }
//
//    async fn get_class(
//        &self, 
//        user_id: Uuid, 
//        class_name: String
//    ) -> Result<Class> {
//        let class_exists = query!(
//            "SELECT 1 FROM classes WHERE user_id = ? AND class_name = ?", 
//            user_id, 
//            class_name
//        )
//        .fetch_optional(&self.pool)
//        .await?
//        .is_some();
//
//        if !class_exists {
//            return Err(RepositoryError::DatabaseError(sqlx::Error::RowNotFound));
//        }
//
//        let schedule_rows = query!(
//            "SELECT schedule_id FROM schedule WHERE class_name = ?", 
//            class_name
//        )
//        .fetch_all(&self.pool)
//        .await?;
//
//        let mut schedules = Vec::new();
//
//        for schedule_row in schedule_rows {
//            let block_rows = query!(
//                "SELECT start_hour, finish_hour, day FROM block WHERE schedule_id = ?", 
//                schedule_row.schedule_id
//            )
//            .fetch_all(&self.pool)
//            .await?;
//
//            let blocks = block_rows.into_iter().map(|row| Block {
//                start_hour: row.start_hour as u8,
//                finish_hour: row.finish_hour as u8,
//                day: Day::from(row.day)
//            }).collect();
//
//            schedules.push(Schedule { blocks });
//        }
//
//        Ok(Class {
//            class_name,
//            schedules
//        })
//    }
//}
