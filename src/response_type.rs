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

impl MultiLiveRoomStatus {
    /// After sending query to the bilibili API, all live room data will be store in
    /// a hashmap, with room id as key, room information as value.
    pub fn data(&self) -> &HashMap<String, LiveRoomInfo> {
        &self.data
    }
}

/// Live room information
#[derive(Serialize, Deserialize, Debug)]
pub struct LiveRoomInfo {
    pub area_name: String,
    pub area_v2_name: String,
    #[serde(with = "live_room_url_serde")]
    pub cover_from_user: Option<Url>,
    #[serde(with = "live_room_url_serde")]
    pub keyframe: Option<Url>,
    pub live_status: LiveStatus,
    pub online: u64,
    #[serde(with = "live_room_tag_name_serde")]
    pub tag_name: Vec<String>,
    pub uname: String,
    pub uid: u64,
    pub title: String,
    pub room_id: u64,
}

/// This mod introduce a custom serialize and deserialize method for casting String to Vec<String>.
/// In bilibili api, tag name are separate with comma `,` and group them in a single string.
/// So when we serialize the json string to our LiveRoomInfo tag, we need a customized ser/de
/// function to split string and join strings.
mod live_room_tag_name_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(tag_name: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&tag_name.join(","))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.split(",").map(|s| s.to_string()).collect())
    }
}

mod live_room_url_serde {
    use serde::{self, de, Deserialize, Deserializer, Serializer};
    use url::Url;

    pub fn serialize<S>(s: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match s {
            Some(u) => serializer.serialize_str(&u.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }

        let url = match url::Url::parse(&s) {
            Ok(u) => u,
            Err(e) => {
                return Err(de::Error::custom(format!(
                    "Unexpected URL format. Parsing URL: {s} Got: {e}"
                )))
            }
        };

        Ok(Some(url))
    }
}

/// Represeting current live room status
#[derive(Debug, PartialEq)]
pub enum LiveStatus {
    /// Represeting u8 number: 0
    Sleep,
    /// Represeting u8 number: 1
    Living,
    /// Represeting u8 number: 2
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

#[test]
fn test_u8_2_live_status() {
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        status: LiveStatus,
    }

    let input = r#" { "status": 1 } "#;
    let output: TestStruct = serde_json::from_str(input).unwrap();
    assert_eq!(output.status, LiveStatus::Living);

    let input = r#" { "status": 2 } "#;
    let output: TestStruct = serde_json::from_str(input).unwrap();
    assert_eq!(output.status, LiveStatus::Loop);

    let input = r#" { "status": 0 } "#;
    let output: TestStruct = serde_json::from_str(input).unwrap();
    assert_eq!(output.status, LiveStatus::Sleep);
}

#[test]
fn test_invalid_live_status_de() {
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        status: LiveStatus,
    }

    use std::panic::catch_unwind as catch;

    let input = r#" { "status": 3 } "#;
    let result = catch(|| serde_json::from_str::<'_, TestStruct>(input).unwrap());
    assert!(result.is_err());

    let input = r#" { "status": -1 } "#;
    let result = catch(|| serde_json::from_str::<'_, TestStruct>(input).unwrap());
    assert!(result.is_err());

    let input = r#" { "status": "foo" } "#;
    let result = catch(|| serde_json::from_str::<'_, TestStruct>(input).unwrap());
    assert!(result.is_err());
}

#[test]
fn test_json_2_live_room() {
    let input = r#"
        {
            "title": "【B限】玩个毛线",
            "room_id": 22637261,
            "uid": 672328094,
            "online": 4087370,
            "live_time": 0,
            "live_status": 2,
            "short_id": 0,
            "area": 6,
            "area_name": "生活娱乐",
            "area_v2_id": 371,
            "area_v2_name": "虚拟主播",
            "area_v2_parent_name": "虚拟主播",
            "area_v2_parent_id": 9,
            "uname": "嘉然今天吃什么",
            "face": "http://i2.hdslb.com/bfs/face/d399d6f5cf7943a996ae96999ba3e6ae2a2988de.jpg",
            "tag_name": "日常,学习,萌宠,厨艺,手机直播",
            "tags": "",
            "cover_from_user": "http://i0.hdslb.com/bfs/live/new_room_cover/f3ed7a782c13086e536ec8bc6e9593bb4918f905.jpg",
            "keyframe": "http://i0.hdslb.com/bfs/live-key-frame/keyframe041722000000226372619dr3m8.jpg",
            "lock_till": "0000-00-00 00:00:00",
            "hidden_till": "0000-00-00 00:00:00",
            "broadcast_type": 0
        }
    "#;

    let info: LiveRoomInfo = serde_json::from_str(input).unwrap();
    assert_eq!(info.title, "【B限】玩个毛线");
    assert_eq!(info.live_status, LiveStatus::Loop);
    assert_eq!(
        info.tag_name,
        vec![
            "日常".to_string(),
            "学习".to_string(),
            "萌宠".to_string(),
            "厨艺".to_string(),
            "手机直播".to_string(),
        ]
    );

    let url = Url::parse(
        "http://i0.hdslb.com/bfs/live/new_room_cover/f3ed7a782c13086e536ec8bc6e9593bb4918f905.jpg",
    )
    .unwrap();
    assert_eq!(info.cover_from_user, Some(url));
}
