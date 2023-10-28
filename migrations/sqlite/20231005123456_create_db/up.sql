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

INSERT INTO "transaction" (username, message, email, day, amount, gems, token, ha_id, receipt_url, event_type, is_mail_sent, is_token_used, is_checked) VALUES ('Brume', 'Coucouuuuu (｡◕‿◕｡)', 'brume@42l.fr', '2023-10-09', 1000, 40, 'blehtoken', 1, 'http://bleh', 'monthlydonation', false, true, false);
INSERT INTO "transaction" (username, message, email, day, amount, gems, token, ha_id, receipt_url, event_type, is_mail_sent, is_token_used, is_checked) VALUES ('Neil', 'BLEHBLEHBLEH', 'neil@42l.fr', '2023-10-19', 1000, 100, 'blehtoken2', 2, 'http://bleh', 'monthlydonation', false, false, false);

INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (1, 95, 42, 1);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (2, 37, 62, 1);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (3, 18, 32, 1);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (4, 30, 42, 1);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (1, 15, 12, 2);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (2, 27, 12, 2);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (4, 11, 32, 2);
INSERT INTO star (startype, position_x, position_y, transactionid) VALUES (1, 35, 72, 2);
