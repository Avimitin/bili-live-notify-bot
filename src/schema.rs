table! {
    chats (id) {
        id -> Int4,
        chat_id -> Int8,
    }
}

table! {
    regis (id) {
        id -> Int4,
        rid -> Int4,
        cid -> Int4,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        room_id -> Int8,
        uname -> Nullable<Text>,
        last_status -> Nullable<Int4>,
        last_query_time -> Nullable<Time>,
    }
}

joinable!(regis -> chats (cid));
joinable!(regis -> rooms (rid));

allow_tables_to_appear_in_same_query!(
    chats,
    regis,
    rooms,
);
