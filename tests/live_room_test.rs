use bili_live_notify::db::Repo;
use bili_live_notify::response_type::LiveStatus;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_get_live_rooms() {
    let conn: PgConnection = {
        dotenv().ok();
        let addr = std::env::var("DATABASE_URL")
            .expect("no integration database address found");
        PgConnection::establish(&addr).expect("fail to connect to pg database")
    };
    let repo: Repo = Repo::new(conn);

    assert_eq!(repo.get_room_status(Some(1), None).await.expect("Fail to get status"), LiveStatus::Living);
}
