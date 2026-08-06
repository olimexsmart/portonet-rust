CREATE TABLE "logs" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "apiname" TEXT NOT NULL,
  "daterequest" TIMESTAMP NOT NULL,
  "params" TEXT
);
