#!/bin/bash
# automeme build and deploy script

# test the binary before proceeding
cargo test
if [ $? != 0 ]; then
    exit
fi

cargo clippy
if [ $? != 0 ]; then
    exit
fi

# build and deploy the docker image
docker build -t automeme .
docker stop automeme
docker rm automeme
docker run -d \
    -p 8888:8888 \
    --restart unless-stopped \
    --name automeme \
    automeme

# tail logs if requested
if [ "$1" = "-f" ]; then
    docker logs automeme -f
fi