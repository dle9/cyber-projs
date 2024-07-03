#!/bin/bash

SCALA_VERSION=${1:-3.4.1}
SBT_VERSION=${2:-1.9.9}

# get scala installer, coursier (cs)
# includes latest versions of scala, sbt, scalac
curl -Ls https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-pc-linux.gz -o cs.gz 

# extract
gzip -d cs.gz \
&& chmod +x cs \
&& yes | ./cs setup

# add it to path
export PATH=$PATH:$HOME/.local/share/coursier/bin

# get target scala,sbt version
cs install scala:$SCALA_VERSION \
&& cs install scalac:$SCALA_VERSION \
&& cs install sbt:$SBT_VERSION

# finish
echo "export PATH=$PATH:$HOME/.local/share/coursier/bin" >> $HOME/.bashrc
