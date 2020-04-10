CREATE TABLE user_sessions(
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL,
    FOREIGN KEY (user_id)
       REFERENCES users (id)
);