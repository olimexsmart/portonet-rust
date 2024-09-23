CREATE TABLE "logs" (
  "id" SERIAL PRIMARY KEY,
  "apiname" TEXT NOT NULL,
  "daterequest" TIMESTAMP NOT NULL,
  "params" TEXT
);


COMMENT ON TABLE logs IS 'Every request to the system is logged here.';
