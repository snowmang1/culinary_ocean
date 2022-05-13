-- Your SQL goes here

CREATE TABLE users (
	id TEXT NOT NULL PRIMARY KEY,
	user_email TEXT NOT NULL,
	password TEXT NOT NULL,
	instructions TEXT NOT NULL,
	ingredients TEXT NOT NULL
)
