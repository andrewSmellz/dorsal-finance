CREATE TABLE accounts (
    id    BLOB PRIMARY KEY,
    name  TEXT NOT NULL UNIQUE,
    kind  TEXT NOT NULL CHECK (kind IN
          ('tfsa','rrsp','fhsa','unregistered','chequing','saving',
           'creditcard','loan','receivable'))--as of 11/16/2026 the receivable account is not implemented
);

CREATE TABLE transactions (
    id           BLOB PRIMARY KEY,
    transfer_id  BLOB,
    account_id   BLOB NOT NULL REFERENCES accounts(id),
    kind         TEXT NOT NULL CHECK (kind IN
                 ('deposit','withdrawal','income','expense','refund',
                  'buy','sell','dividend','interest',
                  'transfer_in','transfer_out','opening_balance')),
    category     TEXT,
    amount       TEXT NOT NULL,
    currency     TEXT NOT NULL,
    ticker       TEXT,
    quantity     TEXT,
    occurred_on  TEXT NOT NULL,
    created_at   TEXT NOT NULL,
    note         TEXT,

    -- every expense or refund has a category, and nothing else does
    CHECK ((kind IN ('expense','refund')) = (category IS NOT NULL))
);

CREATE INDEX idx_tx_account ON transactions(account_id, occurred_on);
CREATE INDEX idx_tx_transfer ON transactions(transfer_id) WHERE transfer_id IS NOT NULL;
-- an account can only have one opening balance per currency
CREATE UNIQUE INDEX idx_one_opening ON transactions(account_id, currency) WHERE kind = 'opening_balance';