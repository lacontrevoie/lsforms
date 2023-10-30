CREATE TABLE "transaction" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL,
    message VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    day TIMESTAMP NOT NULL,
    amount INTEGER NOT NULL,
    gems INTEGER NOT NULL,
    token VARCHAR NOT NULL UNIQUE,
    ha_id INTEGER NOT NULL,
    receipt_url VARCHAR NOT NULL,
    event_type VARCHAR NOT NULL,
    is_mail_sent BOOLEAN NOT NULL,
    is_token_used BOOLEAN NOT NULL,
    is_checked BOOLEAN NOT NULL
);

CREATE TABLE "star" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    startype INTEGER NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    transactionid INTEGER NOT NULL REFERENCES "transaction"
);
