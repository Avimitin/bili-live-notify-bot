use bili_live_notify::{
    db::{PgsqlRepoOperator, RepoOperator},
    response_type::LiveStatus,
};
use dotenv::dotenv;
use std::{future::Future, sync::Arc};

lazy_static::lazy_static! {
    static ref ASYNC_RT: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("fail to build tokio runtime")
    };
    static ref RAW_PG_CONN: Arc<sqlx::PgPool> = {
        dotenv().ok();
        let addr = std::env::var("BILI_NOTIFY_INTEGRATION_TEST_DB")
            .expect("no integration database address found");
        let pool = ASYNC_RT.block_on(sqlx::PgPool::connect(&addr)).expect("fail to connect to pg database");
        Arc::new(pool)
    };
    static ref PG_DB: PgsqlRepoOperator = PgsqlRepoOperator::from_arc(RAW_PG_CONN.clone());
}

fn block_on<F: Future>(fut: F) -> F::Output {
    ASYNC_RT.block_on(fut)
}

async fn clean_up() {
    sqlx::query(r#"DELETE FROM register_relation;"#)
        .execute(&**RAW_PG_CONN)
        .await
        .expect("fail to clean up register_relation");
    sqlx::query(r#"DELETE FROM live_rooms;"#)
        .execute(&**RAW_PG_CONN)
        .await
        .expect("fail to clean up live_rooms");
    sqlx::query(r#"DELETE FROM chats;"#)
        .execute(&**RAW_PG_CONN)
        .await
        .expect("fail to clean up chats");
}

#[test]
fn test_add_live_room() {
    let expect = 1919810;
    let id = block_on(PG_DB.add_live_room(expect)).expect("Fail to add live room");

    let fut = sqlx::query!(r#"SELECT room_id FROM live_rooms WHERE id = $1;"#, id)
        .fetch_one(&**RAW_PG_CONN);
    let ret = block_on(fut).expect("fail to get live room to compare");
    let get = ret.room_id;
    assert_eq!(get, expect);

    block_on(clean_up());
}

#[test]
fn test_get_live_room_status() {
    let room_id = 1919810;
    let expect = LiveStatus::Sleep;
    let qry = sqlx::query!(
        r#"
INSERT INTO live_rooms (room_id, last_status)
VALUES ($1, $2);"#,
        room_id,
        expect.as_str()
    )
    .execute(&**RAW_PG_CONN);
    block_on(qry).expect("fail to insert status into live room");

    let get = block_on(PG_DB.get_live_room_status(room_id)).expect("fail to get live room status");

    assert_eq!(get, expect);

    block_on(clean_up());
}
