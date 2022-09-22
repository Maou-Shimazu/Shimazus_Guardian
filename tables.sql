CREATE TABLE IF NOT EXISTS cases (
    id INTEGER PRIMARY KEY,
    action TEXT,
    moderator_id INTEGER,
    reason TEXT,
    userid INTEGER
);

CREATE TABLE IF NOT EXISTS muted (
    userid INTEGER,
    roles TEXT, -- use split and join
    time INTEGER
);