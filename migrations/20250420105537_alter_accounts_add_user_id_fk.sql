ALTER TABLE accounts
    ADD CONSTRAINT fk_accounts_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (user_id)
            ON DELETE CASCADE;