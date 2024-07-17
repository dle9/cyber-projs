#!/bin/bash

CARGO_VERSION=${1:-1.70.0}

# setup
sudo apt-get update -y \
&& sudo apt-get clean

# install
cd $HOME/tmp
wget https://sh.rustup.rs -O rustup.sh
chmod +x rustup.sh
sudo ./rustup.sh -y

# config
sudo mv /root/.cargo $HOME/
sudo mv /root/.rustup $HOME/
sudo chown -R $APPSEC_USER:$APPSEC_GROUP $HOME/.cargo

# downgrade
sudo $HOME/.cargo/bin/rustup install $CARGO_VERSION
sudo $HOME/.cargo/bin/rustup default $CARGO_VERSION
