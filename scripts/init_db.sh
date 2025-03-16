#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "Error: psql is not installed"
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed"
	echo >&2 "Use:"
	echo >&2 "	cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls, postgres"
	echo >&2 "to install it."
	exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
CONTAINER_NAME="${POSTGRES_DOCKER_CONTAINER_NAME:=z2pdb}"

# launch postgress using Docker if container is not already running
if ! sudo docker ps | grep ${CONTAINER_NAME}; then
	sudo docker rm ${CONTAINER_NAME} || true
	# ^ remove previous container if it exists
	sudo docker run \
		--name ${CONTAINER_NAME} \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p "${DB_PORT}":5432 \
		-d postgres \
		postgres -N 1000 \
		# ^ increased maximum number of connections for testing purposes
fi

export PGPASSWORD="${DB_PASSWORD}"
until pg_isready -h ${DB_HOST} -U ${DB_USER} -p ${DB_PORT} -d "postgres"; do
	echo >&2 "Postgres is still unavailable - sleeping"
	sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT} - running migrations now"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
cargo sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
