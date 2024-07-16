#!/bin/bash

PAKET_VERSION=${1:-8.0.0}

# install
sudo dotnet tool install --global Paket --version $PAKET_VERSION

# config
sudo mv /root/.dotnet $HOME/.dotnet
sudo chown -R $APPSEC_USER:$APPSEC_GROUP $HOME/.dotnet

# nuget is already in dotnet sdk
# ex: dotnet add package <nuget package>