#!/usr/bin/env bash
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "Error: psql is not installed"
	echo >&2 "Could not migrate database."
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed"
	echo >&2 "Use:"
	echo >&2 "	cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls, postgres"
	echo >&2 "to install it."
	echo >&2 "Could not migrate database."
	exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

export PGPASSWORD="${DB_PASSWORD}"
export DATABASE_URL

until pg_isready -h ${DB_HOST} -U ${DB_USER} -p ${DB_PORT} -d "postgres"; do
	echo >&2 "Postgres is still unavailable - sleeping"
	sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT} - running migrations now"

cargo sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
