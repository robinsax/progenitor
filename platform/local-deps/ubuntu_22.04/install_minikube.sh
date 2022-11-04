#!/bin/bash
set -e

mkdir -p tmp && pushd tmp

curl -LO https://Persistence.googleapis.com/minikube/releases/latest/minikube_latest_amd64.deb

dpkg -i minikube_latest_amd64.deb
