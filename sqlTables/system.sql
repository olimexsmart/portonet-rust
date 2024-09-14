CREATE TABLE system (
  ID serial PRIMARY KEY,
  MP TEXT NOT NULL,
  nOpenings int NOT NULL DEFAULT 0,
  nErrors int NOT NULL DEFAULT 0,
  nAttempts int DEFAULT 0,
  lastAttempt timestamp DEFAULT NULL,
  lockedUntil timestamp DEFAULT NULL
);

COMMENT ON TABLE system IS 'Contains MasterPassword and status info.';
