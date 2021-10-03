include .env
export

include Makefile.db.mk

start:
	@cargo run

install:
	# For database migrations.
	@cargo install movine
