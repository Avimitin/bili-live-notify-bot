CREATE TABLE live_status (
  status VARCHAR(5) PRIMARY KEY
);

INSERT INTO live_status(status)
VALUES ('LIVE'), ('SLEEP'), ('LOOP');

CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL
);

CREATE TABLE live_rooms (
  id SERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL UNIQUE,
  uname TEXT,
  last_status VARCHAR(5) REFERENCES live_status(status),
  last_query_time time
);

CREATE TABLE register_relation (
  id SERIAL PRIMARY KEY,
  rid BIGINT NOT NULL REFERENCES live_rooms(id),
  cid BIGINT NOT NULL REFERENCES chats(id)
);
