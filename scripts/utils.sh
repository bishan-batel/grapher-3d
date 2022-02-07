# Constants
BUILD_FOLDER="./server/src/js-build"
LOGFILE="./docker.log"
BUILD_DONE_LINE="https://cra.link/deployment"

# Functions

function container_id() {
    CONTAINER_ID="$(docker ps | grep ib-cs-hl-ia/grapher3d:1.0 -m 1)"

    # If no container ID then failed
    if [ "$CONTAINER_ID" == "" ]; then
        echo ""
        false
    fi

    # echo first id
    echo ${CONTAINER_ID:0:12}
    true
}

function docker_installed() {
    # if command can't find docker then commadn does not exist
    if ! command -v docker &>/dev/null; then
        false
    fi
    true
}
