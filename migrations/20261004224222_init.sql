CREATE TABLE accounts (
    id    BLOB PRIMARY KEY,
    name  TEXT NOT NULL,
    kind  TEXT NOT NULL CHECK (kind IN
          ('tfsa','rrsp','fhsa','unregistered','chequing','saving'))
);

CREATE TABLE transactions (
    id          BLOB PRIMARY KEY,
    account_id  BLOB NOT NULL REFERENCES accounts(id),
    kind        TEXT NOT NULL CHECK (kind IN
                ('deposit','withdrawal','income','expense','buy','sell','dividend','interest')),
    amount      TEXT NOT NULL,
    ticker      TEXT,
    quantity    TEXT,
    timestamp   TEXT NOT NULL,
    note        TEXT
);

CREATE INDEX idx_tx_account ON transactions(account_id, timestamp);