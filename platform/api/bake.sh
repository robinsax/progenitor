#!/bin/bash
set -e

. ../scripts/helpers.sh

msg "baking api as $1"

docker build . \
    -f Dockerfile \
    -t $1 \
    --target runtime
