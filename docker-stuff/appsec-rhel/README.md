### build
podman build --tag rhel:spgl-appsec .

### run
podman run -it rhel:spgl-appsec bash

### remove builds   
podman rmi -a -f