CREATE TABLE user_titles
(
    id        UUID PRIMARY KEY,
    user_id   UUID      NOT NULL,
    title     TEXT      NOT NULL,
    earned_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_user_titles_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (user_id)
            ON DELETE CASCADE
);
