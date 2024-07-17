#!/bin/bash

JAVA_VERSION=${1:-1.8.0}

# install
sudo dnf update -y \
&& sudo dnf install -y java-$JAVA_VERSION-openjdk \
&& sudo dnf clean all

# update certs
sudo dnf install -y ca-certificates \
&& sudo update-ca-trust extract \
&& sudo dnf clean all

# env vars
echo "export JAVA_HOME=/usr/lib/jvm/jre-$JAVA_VERSION-openjdk/" >> $HOME/.bashrc
exec /bin/bash
