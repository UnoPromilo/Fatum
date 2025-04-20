CREATE TABLE quests
(
    id               UUID PRIMARY KEY,
    journey_entry_id UUID      NOT NULL,
    title            TEXT      NOT NULL,
    description      TEXT      NOT NULL,
    status           TEXT      NOT NULL CHECK (status IN ('proposed', 'active', 'completed', 'discarded', 'abandoned')),
    created_at       TIMESTAMP NOT NULL DEFAULT now(),
    updated_at       TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_quests_journey_entry_id
        FOREIGN KEY (journey_entry_id)
            REFERENCES journey_entries (id)
            ON DELETE CASCADE
);
