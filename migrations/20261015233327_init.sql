CREATE TABLE accounts (
    id    BLOB PRIMARY KEY,
    name  TEXT NOT NULL UNIQUE,
    kind  TEXT NOT NULL CHECK (kind IN
          ('tfsa','rrsp','fhsa','unregistered','chequing','saving',
           'creditcard','loan','receivable')),--as of 11/16/2026 the receivable account is not implemented
    allow_negative INTEGER NOT NULL DEFAULT 0 CHECK (allow_negative IN (0, 1))
);

-- a tree: parent_id is NULL for top-level categories
CREATE TABLE categories (
    id         BLOB PRIMARY KEY,
    parent_id  BLOB REFERENCES categories(id),
    name       TEXT NOT NULL COLLATE NOCASE,
    archived   INTEGER NOT NULL DEFAULT 0 CHECK (archived IN (0, 1)),

    -- a name is one level only; '/' is reserved for writing paths like 'Food/Groceries'
    CHECK (name <> '' AND name = trim(name) AND name NOT LIKE '%/%'),
    CHECK (parent_id IS NULL OR parent_id <> id)
);

-- names are unique among siblings; COALESCE because SQLite treats NULLs as distinct
CREATE UNIQUE INDEX idx_category_sibling_name ON categories(COALESCE(parent_id, x''), name);
CREATE INDEX idx_category_parent ON categories(parent_id) WHERE parent_id IS NOT NULL;

INSERT INTO categories (id, name) VALUES
    (randomblob(16), 'Food'),
    (randomblob(16), 'Housing'),
    (randomblob(16), 'Utilities'),
    (randomblob(16), 'Phone'),
    (randomblob(16), 'Insurance'),
    (randomblob(16), 'Health Care'),
    (randomblob(16), 'Clothes'),
    (randomblob(16), 'Home Essentials'),
    (randomblob(16), 'Kitchenware'),
    (randomblob(16), 'Transportation'),
    (randomblob(16), 'Entertainment'),
    (randomblob(16), 'Hobbies'),
    (randomblob(16), 'Gifts'),
    (randomblob(16), 'Misc');

INSERT INTO categories (id, parent_id, name)
SELECT randomblob(16), id, child
FROM categories, (SELECT 'Groceries' AS child UNION ALL SELECT 'Eating Out')
WHERE name = 'Food' AND parent_id IS NULL;

CREATE TABLE transactions (
    id           BLOB PRIMARY KEY,
    transfer_id  BLOB,
    account_id   BLOB NOT NULL REFERENCES accounts(id),
    kind         TEXT NOT NULL CHECK (kind IN
                 ('deposit','withdrawal','income','expense','refund',
                  'buy','sell','dividend','interest',
                  'transfer_in','transfer_out','opening_balance')),
    category_id  BLOB REFERENCES categories(id),
    amount       TEXT NOT NULL,
    currency     TEXT NOT NULL,
    ticker       TEXT,
    quantity     TEXT,
    occurred_on  TEXT NOT NULL,
    created_at   TEXT NOT NULL,
    note         TEXT,

    -- every expense or refund has a category, and nothing else does
    CHECK ((kind IN ('expense','refund')) = (category_id IS NOT NULL))
);

CREATE INDEX idx_tx_account ON transactions(account_id, occurred_on);
CREATE INDEX idx_tx_category ON transactions(category_id) WHERE category_id IS NOT NULL;
CREATE INDEX idx_tx_transfer ON transactions(transfer_id) WHERE transfer_id IS NOT NULL;
-- an account can only have one opening balance per currency
CREATE UNIQUE INDEX idx_one_opening ON transactions(account_id, currency) WHERE kind = 'opening_balance';