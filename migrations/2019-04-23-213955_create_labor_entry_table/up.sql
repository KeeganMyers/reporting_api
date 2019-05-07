CREATE TABLE labor_entry (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  employee_id TEXT NOT NULL,
  name VARCHAR(255) NOT NULL,
  clock_in TIMESTAMPTZ NOT NULL,
  clock_out TIMESTAMPTZ NOT NULL,
  pay_rate NUMERIC NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
