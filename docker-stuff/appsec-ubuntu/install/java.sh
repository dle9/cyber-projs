#!/bin/bash

JAVA_VERSION=${1:-8}

# install
apt-get update \
&& apt-get install -y openjdk-$JAVA_VERSION-jdk \
&& apt-get clean

# env vars
echo "export JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/" >> $HOME/.bashrc
exec /bin/bash
