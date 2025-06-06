#!/usr/bin/env bash

docker buildx build \
    --tag event-tracker \
    --file Dockerfile \
    .
