CREATE TABLE logs (
  ID serial PRIMARY KEY,
  APIName char(15) DEFAULT NULL,
  dateRequest timestamp DEFAULT NULL,
  params varchar(250) DEFAULT NULL
);

COMMENT ON TABLE logs IS 'Every request to the system is logged here.';
