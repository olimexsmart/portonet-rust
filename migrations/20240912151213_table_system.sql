-- Add migration script here
CREATE TABLE system (
  ID serial PRIMARY KEY,
  MP char(20) NOT NULL,
  nOpenings int NOT NULL DEFAULT 0,
  nErrors int NOT NULL DEFAULT 0,
  nAttempts int DEFAULT 0,
  lastAttempt timestamp DEFAULT NULL,
  lockedUntil timestamp DEFAULT NULL
);

CREATE TABLE logs (
  ID serial PRIMARY KEY,
  APIName char(15) DEFAULT NULL,
  dateRequest timestamp DEFAULT NULL,
  params varchar(250) DEFAULT NULL
);


COMMENT ON TABLE system IS 'Contains MasterPassword and status info.';
COMMENT ON TABLE keys IS 'Contains keys used to open door along with supporting data.';
COMMENT ON TABLE logs IS 'Every request to the system is logged here.';

