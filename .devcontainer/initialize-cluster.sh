#!/bin/bash

set -o errexit
set -o pipefail

# Create k3d cluster and forwarded ports (MQTT/MQTTS)
k3d cluster delete
k3d cluster create \
    -p '1883:31883@loadbalancer' \
    -p '8883:38883@loadbalancer' \
    --registry-create registry:0.0.0.0:5000 \
    --wait

# Set the default context / namespace
kubectl config set-context k3d-k3s-default --namespace=azure-iot-operations

# Install Dapr components
dapr init -k
