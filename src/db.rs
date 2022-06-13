use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::response_type::LiveStatus;
use anyhow::Result;

/// RepoOperator represent list of operation to database
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait RepoOperator {
    async fn add_live_room(&self, room_id: i64) -> Result<i32>;
    async fn get_live_room_status(
        &self,
        room_id: i64,
    ) -> Result<LiveStatus, GetLiveRoomStatusError>;
    /// Compare the live status in database, update them and return a list of rooms that has been
    /// modified. The return list can be used as notify parameters.
    async fn update_live_room(&self, room_id: u64, live_status: LiveStatus) -> Result<Vec<u64>>;
    async fn remove_live_room(&self, room_id: u64) -> Result<()>;
    async fn add_chat(&self, chat_id: u64) -> Result<u32>;
    async fn remove_chat(&self, chat_id: u64) -> Result<u32>;
    async fn register(&self, chat_id: u64, room_id: u64) -> Result<u32>;
    async fn unregister(&self, chat_id: u64, room_id: u64) -> Result<u32>;
    async fn get_regis(&self, room_id: u64) -> Result<Vec<u64>>;
}

#[derive(Debug, Clone)]
pub struct PgsqlRepoOperator {
    conn_pool: Arc<PgPool>,
}

impl PgsqlRepoOperator {
    /// Create a new pgsql connection pool which holded by Arc.
    pub fn new(conn_pool: PgPool) -> Self {
        Self {
            conn_pool: Arc::new(conn_pool),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetLiveRoomStatusError {
    #[error("no status found")]
    EmptyRoomStatus,
    #[error("fail to run sql query: {0}")]
    SqlError(sqlx::Error),
}

#[async_trait::async_trait]
impl RepoOperator for PgsqlRepoOperator {
    async fn add_live_room(&self, room_id: i64) -> Result<i32> {
        let qry = sqlx::query!(
            r#"
INSERT INTO live_rooms ( room_id )
VALUES ( $1 )
RETURNING id;
"#,
            room_id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(qry.id)
    }

    async fn get_live_room_status(
        &self,
        room_id: i64,
    ) -> Result<LiveStatus, GetLiveRoomStatusError> {
        let qry = sqlx::query!(
            r#"
SELECT last_status
FROM live_rooms
WHERE room_id = $1;"#,
            room_id
        )
        .fetch_one(&*self.conn_pool)
        .await
        .map_err(|e| GetLiveRoomStatusError::SqlError(e))?;

        let last_status = qry
            .last_status
            .ok_or_else(|| GetLiveRoomStatusError::EmptyRoomStatus)?;
        let status = LiveStatus::from(&last_status).unwrap();
        Ok(status)
    }

    async fn update_live_room(&self, room_id: u64, live_status: LiveStatus) -> Result<Vec<u64>> {
        todo!()
    }
    async fn remove_live_room(&self, room_id: u64) -> Result<()> {
        todo!()
    }
    async fn add_chat(&self, chat_id: u64) -> Result<u32> {
        todo!()
    }
    async fn remove_chat(&self, chat_id: u64) -> Result<u32> {
        todo!()
    }
    async fn register(&self, chat_id: u64, room_id: u64) -> Result<u32> {
        todo!()
    }
    async fn unregister(&self, chat_id: u64, room_id: u64) -> Result<u32> {
        todo!()
    }
    async fn get_regis(&self, room_id: u64) -> Result<Vec<u64>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_live_room() {
        let mut db = MockRepoOperator::new();
        db.expect_add_live_room()
            .times(1)
            .with(mockall::predicate::eq(12345678_i64))
            .returning(|_| Ok(1));
    }
}
