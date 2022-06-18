use chrono::NaiveTime;

#[derive(Queryable)]
pub struct Rooms {
    pub id: i32,
    pub room_id: i64,
    pub uname: Option<String>,
    pub last_status: Option<i32>,
    pub last_query_time: Option<NaiveTime>,
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
