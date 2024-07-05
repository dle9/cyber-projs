#!/bin/bash

NODE_VERSION=${1:-21.x}

# setup
curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION} -o /tmp/nodesource.sh | bash /tmp/nodesource.sh

# install
apt-get update -y \
&& apt-get install -y nodejs \
&& apt-get install -y npm

# cleanup
rm -f /tmp/nodesource.sh
