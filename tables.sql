CREATE TABLE IF NOT EXISTS cases (
    id            INTEGER PRIMARY KEY NOT NULL, 
    action        TEXT                NOT NULL, 
    moderator_id  INTEGER             NOT NULL,
    reason        TEXT                NOT NULL, 
    userid        INTEGER             NOT NULL
);

CREATE TABLE IF NOT EXISTS muted (
    userid  INTEGER NOT NULL, 
    roles   TEXT    NOT NULL
);

