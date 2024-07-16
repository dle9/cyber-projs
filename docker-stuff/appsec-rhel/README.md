### build
podman build --tag ubuntu:spgl-appsec .

### run
podman run -it ubuntu:spgl-appsec bash

### remove images
podman rmi -a -f