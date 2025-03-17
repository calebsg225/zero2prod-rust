#!/usr/bin/env bash
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
CONTAINER_NAME="${POSTGRES_DOCKER_CONTAINER_NAME:=z2pdb}"

# launch postgress using Docker if container is not already running
if ! sudo docker ps | grep ${CONTAINER_NAME}; then
	if ! sudo docker ps -a | grep ${CONTAINER_NAME}; then
		# if container does not exist create new one
		sudo docker run \
			--name ${CONTAINER_NAME} \
			-e POSTGRES_USER=${DB_USER} \
			-e POSTGRES_PASSWORD=${DB_PASSWORD} \
			-e POSTGRES_DB=${DB_NAME} \
			-p "${DB_PORT}":5432 \
			-d postgres \
			postgres -N 1000 \
			# ^ increased maximum number of connections for testing purposes
	else
		# start existing container
		sudo docker start ${CONTAINER_NAME}
	fi
fi

# migrate database
./scripts/migrate_db.sh
