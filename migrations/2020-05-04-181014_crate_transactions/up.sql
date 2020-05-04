CREATE TABLE transactions(
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    date_transaction INTEGER NOT NULL,
    sell_amount INTEGER NOT NULL,
    sell_currency INTEGER NOT NULL,
    buy_amount INTEGER NOT NULL,
    buy_currency INTEGER NOT NULL,
    price_for_one INTEGER NOT NULL,
    FOREIGN KEY (user_id)
       REFERENCES users (id)
);