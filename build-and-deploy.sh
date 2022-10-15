#!/usr/bin/env bash

cd "$(dirname "$0")"
date

echo "## Crash on errors and unset variables ##"
set -e
set -u

# https://console.cloud.google.com/artifacts/docker/haaskme/us-central1/haaskme?project=haaskme

PROJECTID="haaskme"
REPOSITORY="haaskme"
IMAGE="rust-bert-server"
DOCKER_REGISTRY="us-central1-docker.pkg.dev/$PROJECTID/$REPOSITORY/$IMAGE"

docker build --platform=linux/amd64 -t "rust-bert-server" -f Dockerfile .
docker tag haaskme "$DOCKER_REGISTRY"
docker push "$DOCKER_REGISTRY"

# you'll need to redeploy cloudrun afterwards for this to have an effect!
