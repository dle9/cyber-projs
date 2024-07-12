#!/bin/bash

NODE_VERSION=${1:-21.x}

# setup
cd $HOME/tmp
sudo curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION} -o nodesource.sh | bash -

# install
sudo apt-get update -y \
&& sudo apt-get install -y nodejs \
&& sudo apt-get install -y npm

# cleanup
sudo rm -f nodesource.sh
    