#!/bin/bash
set -e

. ../scripts/helpers.sh

declare -a PRG_SERVICES=(
    "frontend"
    "api"
)

VERSIONS_VARS=""
for SERVICE in ${PRG_SERVICES[@]}; do
    VERSION=$(../scripts/get_service_version.sh ../$SERVICE)
    
    msg "$SERVICE@$VERSION"

    VERSIONS_VARS+="    ${SERVICE} = \"$VERSION\"\n"
done

printf "service_versions = {\n$VERSIONS_VARS}" > service_versions.tfvars

terraform apply -var-file="service_versions.tfvars" --parallelism=3
