#!/usr/bin/env bash

docker run \
  --privileged \
  --security-opt no-new-privileges \
  --cap-add=SYS_ADMIN \
  --cap-add=SYS_BPF \
  --cap-add=SYS_PTRACE \
  --cap-add=NET_ADMIN \
  --rm -it -p 127.0.0.1:8080:8080 \
  -w /build \
  -v /lib/modules:/lib/modules:ro \
  -v /usr/src/kernels:/usr/src/kernels:ro \
  -v $PWD:/build metalens \
  /bin/bash -c "cargo run"
