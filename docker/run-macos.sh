#!/usr/bin/env bash

docker run \
  --privileged \
  -it -p 127.0.0.1:8080:8080 \
  -w /build \
  -v $PWD:/build metalens \
  /bin/bash -c "cargo run"
