CREATE TABLE IF NOT EXISTS users (
	id serial PRIMARY KEY,
	name text NOT NULL,
	profile text NULL,
	age int NULL,
	created_at timestamptz NOT NULL DEFAULT current_timestamp,
	updated_at timestamptz NOT NULL DEFAULT current_timestamp,
	deleted_at timestamptz
);
