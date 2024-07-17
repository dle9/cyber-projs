### build
podman build --tag ubuntu:spgl-appsec .

### run
podman run -it ubuntu:spgl-appsec bash

### remove builds
podman rmi -a -f