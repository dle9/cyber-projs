#!bin/bash

GRADLE_VERSION=${1:-7.6.4}
INSTALL_DIR="/usr/share"

# instal
cd $INSTALL_DIR
curl -L https://services.gradle.org/distributions/gradle-$GRADLE_VERSION-bin.zip -o gradle-$GRADLE_VERSION-bin.zip
unzip gradle-$GRADLE_VERSION-bin.zip

# finish
rm gradle-$GRADLE_VERSION-bin.zip
export GRADLE_HOME=$INSTALL_DIR/gradle-$GRADLE_VERSION
echo "export PATH=$PATH:$GRADLE_HOME/bin" >> $HOME/.bashrc
exec /bin/bash
cd $HOME
