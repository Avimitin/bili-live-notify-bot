use crate::db;
use crate::response_type::MultiLiveRoomStatus;
use anyhow::Result;

// TODO: This implementation is simple and naive. Needs to be rewrited by MapReduce Algorithms.
async fn sync(repo: &db::LiveStatusRepo, cfg: &crate::config::Config) -> Result<Vec<i64>> {
    let mut rooms = db::RoomsOperator::get_pending(repo.conn(), cfg.duration()).await?;

    let mut pending_notify = Vec::with_capacity(rooms.len());
    let mut is_drained = false;

    loop {
        let uids: Vec<i64> = if rooms.len() > cfg.query_amount {
            rooms.drain(0..cfg.query_amount).collect()
        } else {
            is_drained = true;
            rooms.drain(..).collect()
        };

        let multi = MultiLiveRoomStatus::new(&uids).await?;

        let mut updated = db::RoomsOperator::update_status(repo.conn(), &multi).await;
        pending_notify.append(&mut updated);

        if is_drained {
            break;
        }
    }

    Ok(pending_notify)
}
