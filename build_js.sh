#!/bin/bash

source ./scripts/utils.sh

# Checks if docker is installed, if not than exit
if ! docker_installed; then
    echo "Docker is not installed"
    echo "https://docs.docker.com/get-docker/"
    exit
fi

# Cleans up any previous containers
bash ./clean.sh

# removes logfile
rm $LOGFILE

# starts up docker container & sends output to logfile
# (all as a background proccess)
nohup bash -c "docker run ib-cs-hl-ia/grapher3d:1.0" >$LOGFILE &

echo "-----------------------------------------------------"
echo "Build started, check ${LOGFILE} for live build output"
echo "-----------------------------------------------------"

# repeat while there is no BUILD_DONE_LINE in file
while [ "$(grep $BUILD_DONE_LINE $LOGFILE)" == "" ]; do
    # if no container ID found then exit with err
    ID="$(container_id)"
    if [ ID == "" ]; then
        echo Build Failed, check $LOGFILE
        exit
    fi
done

id="$(container_id)"
docker cp ${id}:/app/client/build $BUILD_FOLDER

bash ./clean.sh

echo ----
echo "Build succeeded"

# docker run -it ib-cs-hl-ia/grapher3d:1.0
