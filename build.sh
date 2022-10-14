#!/usr/bin/env bash

cd "$(dirname "$0")"
date

echo "## Crash on errors and unset variables ##"
set -e
set -u

docker build --no-cache -t bersling/rust-bert-cpu -f cpu.Dockerfile .
