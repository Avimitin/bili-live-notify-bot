use crate::models::*;
use crate::response_type::LiveStatus;
use crate::schema::rooms;
use diesel::dsl::IntervalDsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum GetRoomQueryParams {
    Id(i32),
    RoomId(i64),
}

impl Display for GetRoomQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetRoomQueryParams::Id(i) => write!(f, "serial id: {i}"),
            GetRoomQueryParams::RoomId(i) => write!(f, "room id: {i}"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RepoOperationError<T: Display> {
    #[error("Fail to get status for {param}: {raw}")]
    GetRoomStatusError { param: GetRoomQueryParams, raw: T },
    #[error("Invalid query parameter")]
    InvalidQueryParamError,
    #[error("No result found: {msg}")]
    NoExpectResultFoundError { msg: String },
    #[error("Fail to get pending rooms: {msg}")]
    GetPendingRoomsError { msg: String, source: T },
}

/// Repo keeps a immutable reference to the PostgreSQL connection pool.
/// It capsulate most of the database operations
#[derive(Clone)]
pub struct Repo {
    conn: Arc<Mutex<PgConnection>>,
}

impl Repo {
    /// Create a new struct to hold the given connection to PostgreSQL.
    pub fn new(conn: PgConnection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    /// Get room status by room id or local id.
    pub async fn get_room_status(
        &self,
        id: Option<i32>,
        room_id: Option<i64>,
    ) -> Result<LiveStatus, RepoOperationError<diesel::result::Error>> {
        let conn = self.conn.lock().await;
        let room = tokio::task::block_in_place(move || {
            if let Some(id) = id {
                rooms::table
                    .filter(rooms::id.eq_all(id))
                    .first::<Rooms>(&*conn)
                    .map_err(|e| RepoOperationError::GetRoomStatusError {
                        param: GetRoomQueryParams::Id(id),
                        raw: e,
                    })
            } else if let Some(rid) = room_id {
                rooms::table
                    .filter(rooms::room_id.eq_all(rid))
                    .first::<Rooms>(&*conn)
                    .map_err(|e| RepoOperationError::GetRoomStatusError {
                        param: GetRoomQueryParams::RoomId(rid),
                        raw: e,
                    })
            } else {
                Err(RepoOperationError::InvalidQueryParamError)
            }
        })?;

        Ok(LiveStatus::from(room.status)
            .expect("Unexpect live status appear, please check database health"))
    }

    /// Get all rooms that has outdated status and is pending for querying.
    pub async fn get_pending_rooms(
        &self,
    ) -> Result<Vec<i64>, RepoOperationError<diesel::result::Error>> {
        let conn = self.conn.lock().await;

        // TODO: make interval value configurable
        let rooms = tokio::task::block_in_place(move || {
            rooms::table
                .filter(rooms::updated_at.gt(diesel::dsl::now + 1.minutes()))
                .select(rooms::room_id)
                .load::<i64>(&*conn)
                .map_err(|e| RepoOperationError::GetPendingRoomsError {
                    msg: "fail to load all pending rooms".to_string(),
                    source: e,
                })
        })?;

        Ok(rooms)
    }

    /// This function should only be used when we are testing/debugging
    #[cfg(debug_assertions)]
    pub async fn raw_execute(&self, query: &str) {
        let conn = self.conn.lock().await;
        diesel::sql_query(query)
            .execute(&*conn)
            .unwrap_or_else(|_| panic!("fail to execute query: {query}"));
    }
}
