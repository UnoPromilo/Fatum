CREATE TABLE accounts (
    user_id uuid PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
)