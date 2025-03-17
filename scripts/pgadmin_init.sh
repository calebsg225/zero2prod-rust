#!/usr/bin/env bash
set -x
set -eo pipefail

PGADMIN_PORT="${PGADMIN_PORT:="5050"}"
PGADMIN_DEFAULT_EMAIL="${PGADMIN_DEFAULT_EMAIL:="admin@admin.com"}"
PGADMIN_DEFAULT_PASSWORD="${PGADMIN_DEFAULT_PASSWORD:="password"}"

PGADMIN_CONTAINER_NAME="${PGADMIN_DOCKER_CONTAINER_NAME:="pga4"}"

if ! sudo docker ps | grep ${PGADMIN_CONTAINER_NAME}; then
	# run saved container if exists
	if sudo docker ps -a | grep ${PGADMIN_CONTAINER_NAME}; then
		sudo docker start ${PGADMIN_CONTAINER_NAME}
		exit 0
	fi
	# run a new docker container with username and password
	sudo docker run \
		--name ${PGADMIN_CONTAINER_NAME} \
		-e PGADMIN_DEFAULT_EMAIL=${PGADMIN_DEFAULT_EMAIL} \
		-e PGADMIN_DEFAULT_PASSWORD=${PGADMIN_DEFAULT_PASSWORD} \
		-p ${PGADMIN_PORT}:80 \
		-d dpage/pgadmin4
fi
