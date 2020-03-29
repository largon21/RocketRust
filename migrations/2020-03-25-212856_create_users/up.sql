CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    nickname TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    token INTEGER
);