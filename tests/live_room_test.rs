use bili_live_notify::db::{PgsqlRepoOperator, RepoOperator};
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

fn drive<F: Future>(fut: F) -> F::Output {
    ASYNC_RT.block_on(fut)
}

#[test]
fn test_add_live_room() {
    let expect = 1919810;
    let id = drive(PG_DB.add_live_room(expect)).expect("Fail to add live room");

    let fut = sqlx::query!(r#"SELECT room_id FROM live_rooms WHERE id = $1"#, id)
        .fetch_one(&**RAW_PG_CONN);
    let ret = drive(fut);

    if ret.is_err() {
        eprintln!("fail to get live room to compare: {:?}", ret);
        return;
    }

    let resp = ret.unwrap();
    let get = resp.room_id;
    assert_eq!(get, expect);
}
