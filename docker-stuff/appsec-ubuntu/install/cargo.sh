#!/bin/bash

CARGO_VERSION=${1:-1.70.0}

# setup
apt-get update -y

# install
cd /tmp
wget https://sh.rustup.rs -O rustup.sh
chmod +x rustup.sh
./rustup.sh -y

# downgrade
$HOME/.cargo/bin/rustup install $CARGO_VERSION
$HOME/.cargo/bin/rustup default $CARGO_VERSION

# finish
cd ..
echo "export PATH=$PATH:$HOME/.cargo/bin" >> $HOME/.bashrc
exec /bin/bash
