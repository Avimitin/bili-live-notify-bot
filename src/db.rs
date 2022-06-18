use crate::models::*;
use crate::response_type::LiveStatus;
use crate::schema::rooms;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::fmt::Display;
use std::sync::Arc;

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
}

/// Repo keeps a immutable reference to the PostgreSQL connection pool.
/// It capsulate most of the database operations
pub struct Repo {
    conn: Arc<PgConnection>,
}

impl Repo {
    /// Create a new struct to hold the given connection to PostgreSQL.
    pub fn new(conn: PgConnection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }

    pub async fn get_room_status(
        &self,
        id: Option<i32>,
        room_id: Option<i64>,
    ) -> Result<LiveStatus, RepoOperationError<diesel::result::Error>> {
        let room = tokio::task::block_in_place(move || {
            if let Some(id) = id {
                rooms::table
                    .filter(rooms::id.eq_all(id))
                    .first::<Rooms>(&*self.conn)
                    .map_err(|e| RepoOperationError::GetRoomStatusError {
                        param: GetRoomQueryParams::Id(id),
                        raw: e,
                    })
            } else if let Some(rid) = room_id {
                rooms::table
                    .filter(rooms::room_id.eq_all(rid))
                    .first::<Rooms>(&*self.conn)
                    .map_err(|e| RepoOperationError::GetRoomStatusError {
                        param: GetRoomQueryParams::RoomId(rid),
                        raw: e,
                    })
            } else {
                return Err(RepoOperationError::InvalidQueryParamError);
            }
        })?;

        let status =
            room.last_status
                .ok_or_else(|| RepoOperationError::NoExpectResultFoundError {
                    msg: format!("No status found for room {}", room.room_id),
                })?;

        Ok(LiveStatus::from(status)
            .expect("Unexpect live status appear, please check database health"))
    }
}
