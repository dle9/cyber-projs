#!/bin/bash

NODE_VERSION=${1:-21.x}

# setup
cd $HOME/tmp
sudo curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION} -o nodesource.sh | bash -

# install
sudo dnf update -y \
&& sudo dnf install -y nodejs \
&& sudo dnf install -y npm \
&& sudo dnf clean all

# cleanup
sudo rm -f nodesource.sh
    