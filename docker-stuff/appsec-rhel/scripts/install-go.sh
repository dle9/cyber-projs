#!/bin/bash

GO_VERSION=${1:-1.21.5}

# setup
cd $HOME/tmp
sudo dnf update -y \
&& sudo dnf clean all

# install 
sudo curl -Ls https://go.dev/dl/go$GO_VERSION.linux-amd64.tar.gz -o go.tar.gz
sudo tar -xzf go.tar.gz
sudo mv go $APPSEC_TOOL_DIR
sudo rm -r go.tar.gz

# config
sudo chown -R $APPSEC_USER:$APPSEC_GROUP $APPSEC_TOOL_DIR/go
