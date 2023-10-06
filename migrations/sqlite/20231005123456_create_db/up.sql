CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR,
    message VARCHAR,
    email VARCHAR NOT NULL,
    time TIMESTAMP NOT NULL,
    amount INTEGER NOT NULL,
    gems INTEGER NOT NULL,
    token VARCHAR NOT NULL,
    is_mail_sent BOOLEAN NOT NULL,
    is_token_used BOOLEAN NOT NULL
);

CREATE TABLE stars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    startype INTEGER NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    transactionid INTEGER NOT NULL REFERENCES transactions
);
