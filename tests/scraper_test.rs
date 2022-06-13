use bili_live_notify::response_type::MultiLiveRoomStatus;

#[tokio::test]
async fn test_multi_live_room_status_scraper() {
    let multi_room = MultiLiveRoomStatus::new(vec![269415357, 730732]).await.unwrap();
    assert_eq!(multi_room.data()["730732"].room_id, 42062);
    assert_eq!(multi_room.data()["269415357"].room_id, 7688602);
}
