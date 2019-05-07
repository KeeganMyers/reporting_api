CREATE TABLE check_tbl (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  employee_id TEXT NOT NULL,
  name VARCHAR(255) NOT NULL,
  closed BOOLEAN NOT NULL,
  closed_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
  );
