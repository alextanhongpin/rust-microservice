up:
	@docker-compose up -d

down:
	@docker-compose down

migrate:
	@movine up

rollback:
	@movine down
