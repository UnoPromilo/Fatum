CREATE TABLE user_tags
(
    id      UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    tag     TEXT NOT NULL,

    CONSTRAINT fk_user_tags_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (user_id)
            ON DELETE CASCADE
);