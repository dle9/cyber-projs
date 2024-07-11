#!/bin/bash

NODE_VERSION=${1:-21.x}

# setup
cd /tmp
curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION} -o /tmp/nodesource.sh | bash /tmp/nodesource.sh

# install
sudo apt-get update -y \
&& sudo apt-get install -y nodejs \
&& sudo apt-get install -y npm

# cleanup
rm -f /tmp/nodesource.sh
