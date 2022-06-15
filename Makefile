.PHONY: setup-dev
setup-dev:
	# Run PostgreSQL
	sudo systemctl start postgresql
	# End Run PostgreSQL
	@echo "BEGIN: setup database"
	@sqlx database drop -y
	@sqlx database create
	@sqlx migrate revert
	@sqlx migrate run
	@echo "END: setup database"

.PHONY: test
test: test_live_room test_scraper

.PHONY: test_live_room
test_live_room:
	@cargo test --test live_room_test -- --test-threads 1

.PHONY: test_scraper
test_scraper:
	@cargo test --test scraper_test
