CREATE TABLE keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ukey TEXT NOT NULL UNIQUE,
    expdate DATETIME NOT NULL,
    lastused DATETIME,
    nused INTEGER NOT NULL DEFAULT 0,
    revoked INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    apiname TEXT NOT NULL,
    daterequest DATETIME NOT NULL,
    params TEXT
);

CREATE TABLE system (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mp TEXT,
    nopenings INTEGER NOT NULL DEFAULT 0,
    nerrors INTEGER NOT NULL DEFAULT 0,
    nattempts INTEGER NOT NULL DEFAULT 0,
    lastattempt DATETIME,
    lockeduntil DATETIME
);

CREATE INDEX logs_daterequest_idx ON logs (daterequest DESC);
