#!/bin/bash

GO_VERSION=${1:-1.21.5}

# install
cd /tmp
apt-get update -y
curl -Ls https://go.dev/dl/go$GO_VERSION.linux-amd64.tar.gz -o go.tar.gz

# extract
tar -C /usr/local -xzf go.tar.gz

# finish
cd ..
echo "export PATH=$PATH:/usr/local/go/bin" >> $HOME/.bashrc
exec /bin/bash
