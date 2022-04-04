CREATE TABLE IF NOT EXISTS Tasks (
    db_id INTEGER PRIMARY KEY NOT NULL,
    public_id INTEGER UNIQUE NOT NULL,
    summary TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL,
    details TEXT,
    alarm_time DATETIME,
    created_time DATETIME
);

CREATE INDEX IF NOT EXISTS status_idx ON Tasks (status);