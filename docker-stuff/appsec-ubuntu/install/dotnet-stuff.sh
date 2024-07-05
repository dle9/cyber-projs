#!/bin/bash

PAKET_VERSION=${1:-8.0.0}

# setup
dotnet tool install --global Paket --version $PAKET_VERSION

# finish
echo "export PATH=$PATH:/root/.dotnet/tools" >> $HOME/.bashrc
exec /bin/bash

# nuget is already in dotnet sdk
# ex: dotnet add package <nuget package>