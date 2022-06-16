CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL
);

CREATE TABLE live_rooms (
  id SERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL UNIQUE,
  uname TEXT,
  last_status INT,
  last_query_time TIME
);

CREATE TABLE register_relation (
  id SERIAL PRIMARY KEY,
  rid BIGINT NOT NULL REFERENCES live_rooms(id),
  cid BIGINT NOT NULL REFERENCES chats(id)
);
