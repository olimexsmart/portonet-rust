CREATE TABLE "system" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "mp" TEXT,
  "nopenings" INTEGER NOT NULL DEFAULT 0,
  "nerrors" INTEGER NOT NULL DEFAULT 0,
  "nattempts" INTEGER NOT NULL DEFAULT 0,
  "lastattempt" TIMESTAMP,
  "lockeduntil" TIMESTAMP
);
