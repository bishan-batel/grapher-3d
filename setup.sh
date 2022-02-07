#!/bin/bash
source ./scripts/utils.sh

# Checks if docker is installed, if not than exit
if ! docker_installed; then
    echo "Docker is not installed"
    echo "https://docs.docker.com/get-docker/"
    exit
fi 

echo "Building Dockerfile, please wait until it is fully complete"

# Builds dockerfile 
docker build -t ib-cs-hl-ia/grapher3d:1.0 .

echo
echo "Dockerfile has been built"