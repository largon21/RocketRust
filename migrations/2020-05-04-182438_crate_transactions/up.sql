CREATE TABLE transactions(
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    date_transaction TEXT NOT NULL,
    sell_amount INTEGER NOT NULL,
    sell_currency TEXT NOT NULL,
    buy_amount INTEGER NOT NULL,
    buy_currency TEXT NOT NULL,
    price_for_one INTEGER NOT NULL,
    FOREIGN KEY (user_id)
       REFERENCES users (id)
);