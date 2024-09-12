CREATE TABLE keys (
  ID serial PRIMARY KEY,
  uKey char(20) NOT NULL,
  expDate timestamp DEFAULT NULL,
  lastUsed timestamp DEFAULT NULL,
  nUsed int NOT NULL DEFAULT 0,
  revoked smallint NOT NULL DEFAULT 0,
  UNIQUE (uKey)
);
