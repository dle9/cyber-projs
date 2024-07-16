#!/bin/bash

GO_VERSION=${1:-1.21.5}

# install
cd $HOME/tmp
sudo dmf update -y
sudo curl -Ls https://go.dev/dl/go$GO_VERSION.linux-amd64.tar.gz -o go.tar.gz

# extract
sudo tar -xzf go.tar.gz
sudo mv go $APPSEC_TOOL_DIR
sudo rm -r go.tar.gz

# config
sudo chown -R $APPSEC_USER:$APPSEC_GROUP $APPSEC_TOOL_DIR/go
