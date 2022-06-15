use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::response_type::LiveStatus;
use anyhow::Result;

/// RepoOperator represent list of operation to database
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
    async fn get_all_live_rooms(&self) -> Result<Vec<i64>>;
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

    /// Create a repo operator by cloning pgpool holding by arc
    pub fn from_arc(conn_pool: Arc<PgPool>) -> Self {
        Self { conn_pool }
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
        .map_err(GetLiveRoomStatusError::SqlError)?;

        let last_status = qry
            .last_status
            .ok_or(GetLiveRoomStatusError::EmptyRoomStatus)?;
        let status = LiveStatus::from(&last_status).unwrap();
        Ok(status)
    }

    async fn update_live_room(&self, room_id: u64, live_status: LiveStatus) -> Result<Vec<u64>> {
        todo!()
    }
    async fn remove_live_room(&self, room_id: u64) -> Result<()> {
        todo!()
    }

    async fn get_all_live_rooms(&self) -> Result<Vec<i64>> {
        let res = sqlx::query!(
            r#"
SELECT (room_id)
FROM live_rooms;
"#
        )
        .fetch_all(&*self.conn_pool)
        .await;

        match res {
            Ok(rec) => Ok(rec.iter().map(|r| r.room_id).collect()),
            Err(sqlx::Error::RowNotFound) => Ok(Vec::new()),
            Err(e) => {
                anyhow::bail!("fail to get all live rooms: {}", e)
            }
        }
    }
}
