#!/bin/bash

source ./scripts/utils.sh

# Checks if docker is installed
if ! docker_installed; then
    echo "Docker is not installed"
    echo "https://docs.docker.com/get-docker/"
    exit
fi

# loops until no processes found
while [ "$(container_id)" != "" ]; do
    # kills process & logs
    id="$(container_id)"
    docker kill $id
    echo Killed container process ${id}
done
