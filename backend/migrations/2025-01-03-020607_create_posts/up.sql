CREATE TABLE evaluations (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  class TEXT NOT NULL,
  date TIMESTAMP NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  login VARCHAR UNIQUE NOT NULL ,
  password_hash VARCHAR NOT NULL
);

CREATE TABLE suggestion_type (
  id INT PRIMARY KEY,
  name varchar not null
);


CREATE TABLE suggestion (
  id SERIAL PRIMARY KEY,
  suggestion_type_id INTEGER NOT NULL REFERENCES suggestion_type(id),
  evaluation_id INTEGER NULL REFERENCES evaluations(id) ON DELETE CASCADE,
  title VARCHAR NULL,
  class VARCHAR NULL,
  date TIMESTAMP NULL
);
INSERT INTO users(login, password_hash) values
  ('admin', 'senha');

INSERT INTO suggestion_type values 
  (1, 'create'),
  (2, 'update'),
  (3, 'delete'); 