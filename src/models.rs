use crate::schema::rooms;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[table_name = "rooms"]
pub struct Rooms {
    pub id: i32,
    pub room_id: i64,
    pub uname: Option<String>,
    pub status: i32,
    pub updated_at: NaiveDateTime,
    pub archive: Option<bool>,
}

#[derive(Queryable)]
pub struct Chats {
    pub id: i32,
    pub room_id: i64,
}

#[derive(Queryable)]
pub struct Regis {
    pub id: i32,
    pub cid: i32,
    pub rid: i32,
}
