#!/bin/bash
set -e

pushd $1 > /dev/null
VERSION=$(./version.sh)
SHA_SUM=$(find . -type f \( -exec sha1sum "$PWD"/{} \; \) | awk '{print $1}' | sort | sha1sum)
popd > /dev/null

echo "$VERSION.${SHA_SUM:0:8}${SHA_SUM:32:8}"
