include .env
export

start:
	@cargo run

install:
	# For database migrations.
	@cargo install movine

migrate:
	@movine up

rollback:
	@movine down
