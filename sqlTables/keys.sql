CREATE TABLE keys (
  ID serial PRIMARY KEY,
  uKey TEXT NOT NULL,
  expDate timestamp DEFAULT NULL,
  lastUsed timestamp DEFAULT NULL,
  nUsed int NOT NULL DEFAULT 0,
  revoked smallint NOT NULL DEFAULT 0,
  UNIQUE (uKey)
);

COMMENT ON TABLE keys IS 'Contains keys used to open door along with supporting data.';
