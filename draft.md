# Database Layout

- live_rooms

| ID | Room ID | Last Status: ref last_status(status) | Last Query Time |

- register_metadata

| ID | rid: ref live_rooms(room_id) | cid: ref chats(chat_id) |

- chats

| ID | Chat ID |

- last_status

| status |
