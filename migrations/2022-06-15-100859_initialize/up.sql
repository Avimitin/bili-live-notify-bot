CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL
);

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,          -- unique id in local
  room_id BIGINT NOT NULL UNIQUE, -- room id for fetching
  uname TEXT,                     -- room's owner user name
  last_status INT,                -- room's last status
  last_query_time TIME,           -- last query time for this rooms
  archive BOOLEAN                 -- true if no chats register this room
);

CREATE TABLE regis (
  id SERIAL PRIMARY KEY,
  rid INT NOT NULL REFERENCES rooms(id),
  cid INT NOT NULL REFERENCES chats(id)
);
