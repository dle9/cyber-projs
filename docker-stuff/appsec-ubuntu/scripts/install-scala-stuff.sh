#!/bin/bash

SCALA_VERSION=${1:-3.3.3}
SBT_VERSION=${2:-1.9.0}

# get scala installer, coursier (cs)
# includes latest versions of scala, sbt, scalac
sudo apt-get update -y
curl -Ls https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-pc-linux.gz -o cs.gz 

# extract
gzip -d cs.gz \
&& sudo chmod +x cs \
&& yes | ./cs setup

# install target scala,sbt version
sudo $HOME/.local/share/coursier/bin/cs install scala:$SCALA_VERSION \
&& sudo $HOME/.local/share/coursier/bin/cs install scalac:$SCALA_VERSION \
&& sudo $HOME/.local/share/coursier/bin/cs install sbt:$SBT_VERSION

# finish
echo "export PATH=$PATH:$HOME/.local/share/coursier/bin" >> $HOME/.bashrc
exec /bin/bash