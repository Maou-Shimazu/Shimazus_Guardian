CREATE TABLE
  IF NOT EXISTS mute (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    is_muted BOOLEAN NOT NULL DEFAULT FALSE
  );

CREATE TABLE
  IF NOT EXISTS ban (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    reason TEXT NOT NULL,
    TIME TIMESTAMP NOT NULL,
    duration BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
  );

CREATE TABLE
  IF NOT EXISTS cases (
    case_id INTEGER PRIMARY KEY AUTOINCREMENT,
    case_action TEXT,
    case_user TEXT,
    case_moderator TEXT,
    case_reason TEXT,
    case_time TEXT
  );