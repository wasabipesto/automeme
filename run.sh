#!/bin/bash
# automeme-web build and deploy script

# build and deploy the docker image
docker build -t automeme-web . || exit
docker stop automeme-web || exit
docker rm automeme-web || exit
docker run -d \
    -p 8888:8888 \
    --restart unless-stopped \
    --name automeme-web \
    automeme-web

# tail logs if requested
if [ "$1" = "-f" ]; then
    docker logs automeme-web -f
fi