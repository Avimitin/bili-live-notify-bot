use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::response_type::LiveStatus;

#[derive(Debug, Default)]
pub struct Room {
    room_id: i64,
    status: Option<LiveStatus>,
    username: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum DbOperationError {
    #[error("Unexpected error: {source}")]
    UnexpectedError {
        #[source]
        source: sqlx::Error,
    },

    #[error("No any primary id or room id specify for query")]
    NoIdForRoomsError,

    #[error("No result for given qualification")]
    NoResult,
}

#[derive(Clone, Debug)]
pub struct RoomsOperator;

impl RoomsOperator {
    /// Get room status by local db primary id OR room id.
    ///
    /// Return error when:
    ///     * neither primary id nor room id was given.
    ///     * no result found
    ///     * internal db error (call error.source() to trace error)
    pub async fn get_status_by_id(
        conn: &PgPool,
        pid: Option<i32>,
        rid: Option<i64>,
    ) -> Result<Room, DbOperationError> {
        if pid.is_none() && rid.is_none() {
            return Err(DbOperationError::NoIdForRoomsError);
        }

        let wrap_err = |oe: sqlx::Error| match oe {
            sqlx::Error::RowNotFound => DbOperationError::NoResult,
            _ => DbOperationError::UnexpectedError { source: oe },
        };

        if pid.is_none() {
            let rid = rid.unwrap();

            let resp = sqlx::query!(r#"SELECT status, uname FROM rooms WHERE room_id = $1"#, rid)
                .fetch_one(conn)
                .await
                .map_err(wrap_err)?;

            return Ok(Room {
                room_id: rid,
                status: Some(resp.status.into()),
                username: resp.uname,
            });
        }

        let pid = pid.unwrap();

        let resp = sqlx::query!(
            r#"SELECT room_id, status, uname FROM rooms WHERE id = $1"#,
            pid
        )
        .fetch_one(conn)
        .await
        .map_err(wrap_err)?;

        return Ok(Room {
            room_id: resp.room_id,
            status: Some(resp.status.into()),
            username: resp.uname,
        });
    }

    pub async fn get_pending(
        conn: &PgPool,
        dur: &chrono::Duration,
    ) -> Result<Vec<i64>, DbOperationError> {
        let dur: sqlx::postgres::types::PgInterval = (*dur)
            .try_into()
            .expect("Fail to convert chrono Duration to PostgreSQL Interval, please check the code in line 117.");

        let err_wrapper = |oe: sqlx::Error| match oe {
            sqlx::Error::RowNotFound => DbOperationError::NoResult,
            _ => DbOperationError::UnexpectedError { source: oe },
        };

        let resp = sqlx::query!(
            r#"SELECT room_id FROM rooms WHERE last_query_at < NOW() - $1::interval;"#,
            dur
        )
        .fetch_all(conn)
        .await
        .map_err(err_wrapper)?;

        Ok(resp.iter().map(|r| r.room_id).collect())
    }

    pub async fn update_status(
        conn: &PgPool,
        new: &crate::response_type::MultiLiveRoomStatus,
    ) -> Vec<i64> {
        let new_data: Vec<(i64, i32)> = new
            .data()
            .values()
            .map(|room| (room.room_id, room.live_status.to_i32()))
            .collect();

        // transaction will be commited after the _transaction_guard value droped
        let _transaction_guard = conn.begin().await.expect("fail to start transaction");

        let mut updated = Vec::with_capacity(new_data.len());

        // NOTES: This implementation might have performance issue.
        // A better implementation is still needed.
        for data in &new_data {
            let row = sqlx::query!(
                r#"
UPDATE rooms
SET status = $1, updated_at = NOW()
WHERE room_id = $2 AND status != 1
RETURNING room_id
"#,
                data.1,
                data.0
            )
            .fetch_optional(conn)
            .await
            .expect("fail to update room status");

            if let Some(row) = row {
                updated.push(row.room_id)
            }
        }

        updated
    }
}

#[derive(Clone, Debug)]
pub struct Chats;
#[derive(Clone, Debug)]
pub struct Regis;

#[derive(Clone, Debug)]
pub struct LiveStatusRepo {
    connection_pool: Arc<PgPool>,
}

impl LiveStatusRepo {
    /// Create a new repository operator with given PostgreSQL connection.
    pub fn new(conn: PgPool) -> Self {
        Self {
            connection_pool: Arc::new(conn),
        }
    }

    /// Immutably access the connection
    pub fn conn(&self) -> &PgPool {
        &self.connection_pool
    }
}
