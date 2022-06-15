#!/bin/bash
set -ex
# This is a development environment setup script

# Run PostgreSQL
sudo systemctl start postgresql
# End Run PostgreSQL

# Make clean tables
DB_URL=postgres://$(whoami)@localhost/bili-notify-dev-db
sqlx database drop -y -D $DB_URL
sqlx database create -D $DB_URL
sqlx migrate revert
sqlx migrate run
# End Run PostgreSQL
