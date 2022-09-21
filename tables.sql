CREATE TABLE IF NOT EXISTS cases (
    id INTEGER PRIMARY KEY,
    action TEXT,
    moderator TEXT,
    reason TEXT,
    userid INTEGER
);