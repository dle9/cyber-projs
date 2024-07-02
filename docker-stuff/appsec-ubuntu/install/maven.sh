#!/bin/bash

MAVEN_VERSION=${1:-3.9.7}
INSTALL_URL=https://apache.osuosl.org/maven/maven-3/${MAVEN_VERSION}/binaries

# install
apt-get -y update && \
mkdir -p /usr/share/maven /usr/share/maven/ref && \
curl -fsSL -o /tmp/apache-maven.tar.gz ${INSTALL_URL}/apache-maven-${MAVEN_VERSION}-bin.tar.gz

# extract
tar --no-same-owner -xzf /tmp/apache-maven.tar.gz -C /usr/share/maven --strip-components=1 && \

# clean up and finish
rm -f /tmp/apache-maven.tar.gz && \
ln -s /usr/share/maven/bin/mvn /usr/bin/mvn

# env vars
echo "export MAVEN_HOME=/usr/share/maven" >> $HOME/.bashrc
echo "export MAVEN_CONFIG=$HOME/.m2" >> $HOME/.bashrc 
echo "export PATH=$PATH:$MAVEN_HOME/bin" >> $HOME/.bashrc
exec /bin/bash
