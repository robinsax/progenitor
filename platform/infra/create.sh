#!/bin/bash
set -e

minikube start --kubernetes-version=v1.23.7

kubectl config use-context minikube

minikube addons enable ingress

terraform init

terraform get --update=true

./apply.sh