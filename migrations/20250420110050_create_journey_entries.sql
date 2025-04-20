CREATE TABLE journey_entries
(
    id          UUID PRIMARY KEY,
    user_id     UUID      NOT NULL,
    title       TEXT      NOT NULL,
    description TEXT      NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_journey_entries_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (user_id)
            ON DELETE CASCADE
);
