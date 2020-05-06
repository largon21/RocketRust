CREATE TABLE transactions(
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    date_transaction TEXT NOT NULL,
    sell_amount REAL NOT NULL,
    sell_currency TEXT NOT NULL,
    buy_amount REAL NOT NULL,
    buy_currency TEXT NOT NULL,
    price_for_one REAL NOT NULL,
    FOREIGN KEY (user_id)
       REFERENCES users (id)
);