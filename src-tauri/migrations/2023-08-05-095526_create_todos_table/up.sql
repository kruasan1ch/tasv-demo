-- Your SQL goes here
CREATE TABLE todos (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL DEFAULT '',
  done BOOLEAN NOT NULL DEFAULT 'f'
);