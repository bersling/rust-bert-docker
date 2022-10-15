#!/usr/bin/env bash

cd "$(dirname "$0")"
date

echo "## Crash on errors and unset variables ##"
set -e
set -u

docker stop rust-bert
docker system prune -y
docker build --no-cache --name rust-bert -t bersling/rust-bert-cpu -f cpu.Dockerfile .
