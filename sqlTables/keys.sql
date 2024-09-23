CREATE TABLE "keys" (
  "id" SERIAL PRIMARY KEY,
  "ukey" TEXT NOT NULL,
  "expdate" TIMESTAMP NOT NULL,
  "lastused" TIMESTAMP,
  "nused" INTEGER NOT NULL DEFAULT 0,
  "revoked" SMALLINT NOT NULL DEFAULT 0,
  UNIQUE ("ukey")
);


COMMENT ON TABLE keys IS 'Contains keys used to open door along with supporting data.';
