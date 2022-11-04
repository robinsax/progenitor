#!/bin/bash
set -e

kubectl port-forward -n persistence svc/mongo 27017:27017
