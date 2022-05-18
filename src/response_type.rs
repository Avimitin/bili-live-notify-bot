use serde::{de, Deserialize, Serialize};
use std::collections::HashMap;
use std::u8;
use url::Url;

/// live room status for multiple room queries
/// Ref: https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/live/info.md#%E6%89%B9%E9%87%8F%E6%9F%A5%E8%AF%A2%E7%9B%B4%E6%92%AD%E9%97%B4%E7%8A%B6%E6%80%81
#[derive(Serialize, Deserialize)]
pub struct MultiLiveRoomStatus {
    code: i32,
    message: String,
    data: HashMap<String, LiveRoomInfo>,
}

/// Live room information
#[derive(Serialize, Deserialize)]
pub struct LiveRoomInfo {
    area_name: String,
    area_v2_name: String,
    cover_from_user: Url,
    keyframe: Url,
    live_status: LiveStatus,
    online: u64,
    tag_name: Vec<String>,
    uname: String,
    uid: u64,
    title: String,
}

/// Represeting current live room status
pub enum LiveStatus {
    Sleep,
    Living,
    Loop,
}

impl Serialize for LiveStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LiveStatus::Sleep => serializer.serialize_u8(0),
            LiveStatus::Living => serializer.serialize_u8(1),
            LiveStatus::Loop => serializer.serialize_u8(2),
        }
    }
}

impl<'de> Deserialize<'de> for LiveStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = u8::deserialize(deserializer)?;
        let stat = match id {
            0 => LiveStatus::Sleep,
            1 => LiveStatus::Living,
            2 => LiveStatus::Loop,
            _ => {
                return Err(de::Error::custom(format!(
                    "Unexpected live status code, expect number 0,1,2, get {id}"
                )))
            }
        };

        Ok(stat)
    }
}
