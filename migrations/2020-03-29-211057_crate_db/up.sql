CREATE TABLE user_sessions(
    id INTEGER PRIMARY KEY,
    user_id INTEGER,
    token TEXT NOT NULL
);

CREATE TABLE users(
    id INTEGER PRIMARY KEY,
    nickname TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);



