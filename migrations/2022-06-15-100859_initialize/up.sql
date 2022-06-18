CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL
);

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL UNIQUE,
  uname TEXT,
  last_status INT,
  last_query_time TIME
);

CREATE TABLE regis (
  id SERIAL PRIMARY KEY,
  rid INT NOT NULL REFERENCES rooms(id),
  cid INT NOT NULL REFERENCES chats(id)
);
