use std::collections::HashMap;

use crate::response_type::MultiLiveRoomStatus;
use anyhow::{Context, Result};
use reqwest;

impl MultiLiveRoomStatus {
    /// This function requires a sequence of the live room id, and then get multiple room
    /// information and parse them into a HashMap, wrapped in struct `MultiLiveRoomStatus`.
    ///
    /// Return error if request fail, response code is not 200, response is empty text,
    /// response is not in JSON format, or the deserialize process fail.
    pub async fn new(uids: &[i64]) -> Result<Self> {
        let param = HashMap::from([("uids", &uids)]);

        let client = reqwest::Client::new();
        let resp = client
            .post("http://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids")
            .json(&param)
            .send()
            .await
            .with_context(|| "Send request to `get_status_info_by_uids` fail")?;

        if resp.status() != reqwest::StatusCode::OK {
            anyhow::bail!("Response is not 200")
        }

        let resp_bytes = resp
            .bytes()
            .await
            .with_context(|| format!("fail to get multi live room info with uids: {:?}", uids))?;

        serde_json::from_slice(&resp_bytes)
            .with_context(|| "fail to transcribe response to MultiLiveRoomStatus")
    }
}
