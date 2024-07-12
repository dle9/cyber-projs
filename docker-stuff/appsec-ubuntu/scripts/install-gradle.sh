#!/bin/bash

GRADLE_VERSION=${1:-7.6.4}

# instal
cd $HOME/tmp
sudo curl -L https://services.gradle.org/distributions/gradle-$GRADLE_VERSION-bin.zip -o gradle.zip
sudo unzip gradle.zip

# finish
sudo rm gradle.zip
sudo mv gradle-$GRADLE_VERSION $APPSEC_TOOL_DIR/gradle