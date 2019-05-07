CREATE TABLE ordered_item (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  employee_id TEXT NOT NULL,
  check_id TEXT NOT NULL,
  item_id TEXT NOT NULL,
  cost NUMERIC DEFAULT 0 NOT NULL,
  price NUMERIC DEFAULT 0 NOT NULL,
  voided BOOLEAN NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
