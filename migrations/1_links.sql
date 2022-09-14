CREATE TABLE "links" (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  hash VARCHAR(16) NOT NULL UNIQUE,
  original_url VARCHAR(512) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMPTZ NOT NULL
);
