CREATE TABLE "system" (
  "id" SERIAL PRIMARY KEY,
  "mp" TEXT,
  "nopenings" INTEGER NOT NULL DEFAULT 0,
  "nerrors" INTEGER NOT NULL DEFAULT 0,
  "nattempts" INTEGER NOT NULL DEFAULT 0,
  "lastattempt" TIMESTAMP,
  "lockeduntil" TIMESTAMP
);

COMMENT ON TABLE system IS 'Contains MasterPassword and status info.';
