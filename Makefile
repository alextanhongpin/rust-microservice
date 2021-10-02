include .env
export

DATABASE_URL=postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}


include Makefile.db.mk

start:
	@cargo run

install:
	# For database migrations.
	@cargo install movine
