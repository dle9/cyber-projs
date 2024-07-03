#!/bin/bash

GO_VERSION=${1:-1.22.4}

# install
curl -Ls https://go.dev/dl/go$GO_VERSION.linux-amd64.tar.gz -o go.tar.gz

# extract
tar -C /usr/local -xzf go.tar.gz

# finish
echo "export PATH=$PATH:/usr/local/go/bin" >> $HOME/.bashrc
exec /bin/bash
