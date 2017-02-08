CREATE TABLE users (
  matrix_user_id VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  language VARCHAR NOT NULL,
  is_virtual_user BOOLEAN NOT NULL,
  last_message_sent BIG INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT users_pk PRIMARY KEY (matrix_user_id)
);
