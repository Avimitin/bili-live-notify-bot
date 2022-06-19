use std::future::Future;

/// TEST SHOULD BE RUN INDIVIDUALLY
/// USE `test-db.sh` script! DON'T RUN `cargo test`
use bili_live_notify::db::Repo;
use bili_live_notify::response_type::LiveStatus;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use tokio::runtime::Runtime;

lazy_static::lazy_static! {
    static ref RT: Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .build()
            .expect("fail to build tokio runtime builder")
    };
    static ref DB: Repo = {
        let conn: PgConnection = {
            dotenv().ok();
            let addr = std::env::var("DATABASE_URL").expect("no integration database address found");
            PgConnection::establish(&addr).expect("fail to connect to pg database")
        };
        Repo::new(conn)
    };
}

fn block<F: Future>(f: F) -> F::Output {
    RT.block_on(f)
}

fn block_execute(s: &str) {
    block(async {
        DB.raw_execute(s).await;
    })
}

fn clean_up() {
    block(async {
        DB.raw_execute("DELETE FROM chats;").await;
        DB.raw_execute("DELETE FROM rooms;").await;
        DB.raw_execute("DELETE FROM regis;").await;
    });
}

#[test]
fn test_get_live_rooms() {
    clean_up();

    block_execute(
        r#"
INSERT INTO rooms (room_id, status, updated_at)
VALUES (12345, 1, NOW());
"#,
    );

    assert_eq!(
        block(DB.get_room_status(None, Some(12345))).expect("Fail to get status"),
        LiveStatus::Living
    );
}

#[test]
fn test_get_pending_rooms() {
    clean_up();

    block_execute(
        r#"
INSERT INTO rooms (room_id, status, updated_at)
VALUES (12345, 1, NOW() + interval '2' minute), (67890, 0, NOW() + interval '2' minute);
"#,
    );

    assert_eq!(
        block(DB.get_pending_rooms()).expect("Fail to get pending rooms"),
        vec![12345, 67890]
    )
}
