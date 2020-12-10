-- Your SQL goes here

CREATE TABLE sensitive_regular (
  id SERIAL PRIMARY KEY,
  regulars VARCHAR(100) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)

SELECT diesel_manage_updated_at('sensitive_regular');