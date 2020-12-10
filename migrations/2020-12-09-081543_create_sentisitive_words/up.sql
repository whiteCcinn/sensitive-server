-- Your SQL goes here
CREATE TABLE sensitive_words (
  id SERIAL PRIMARY KEY,
  words VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)

SELECT diesel_manage_updated_at('sensitive_words');