CREATE TABLE user_charms
(
    id          UUID PRIMARY KEY,
    user_id     UUID      NOT NULL,
    charm_id    TEXT      NOT NULL,
    assigned_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_user_charms_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (user_id)
            ON DELETE CASCADE
);