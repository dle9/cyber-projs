#!/bin/bash

JAVA_VERSION=${1:-8}

# install
sudo apt-get update -y \
&& sudo apt-get install -y openjdk-$JAVA_VERSION-jdk \
&& sudo apt-get clean

# update certs
sudo apt-get install -y ca-certificates-java \
&& sudo update-ca-certificates

# env vars
echo "export JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/" >> $HOME/.bashrc
exec /bin/bash
