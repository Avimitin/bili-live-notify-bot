.PHONY: setup-dev
setup-dev: init-pgsql init-db

.PHONY: init-pgsql
init-pgsql:
	# Run PostgreSQL
	sudo systemctl start postgresql

.PHONY: init-db
init-db:
	# Run sqlx migration
	@sqlx db drop -y
	@sqlx db create
	@sqlx migrate run

.PHONY: test
test: test_live_room test_scraper

.PHONY: test_live_room
test_live_room:
	@cargo test --test live_room_test -- --test-threads 1

.PHONY: test_scraper
test_scraper:
	@cargo test --test scraper_test
