-- Add up migration script here

CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL
);

CREATE TABLE rooms (
  id            SERIAL PRIMARY KEY,             -- unique id in local
  room_id       BIGINT NOT NULL UNIQUE,         -- room id for fetching
  uname         TEXT,                           -- room's owner user name
  status        INT NOT NULL,                   -- room's last status
  last_query_at TIMESTAMP,                      -- last query time for this rooms
  updated_at    TIMESTAMP NOT NULL,             -- last update time for this row
  archive       BOOLEAN NOT NULL DEFAULT FALSE  -- true if no chats register this room
);

CREATE TABLE regis (
  id SERIAL PRIMARY KEY,
  rid INT NOT NULL REFERENCES rooms(id),
  cid INT NOT NULL REFERENCES chats(id)
);
