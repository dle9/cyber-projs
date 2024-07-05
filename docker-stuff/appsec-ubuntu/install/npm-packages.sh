#!/bin/bash

YARN_VERSION=${1:-1.21.1}
BOWER_VERSION=${2:-1.8.14}

# install
npm install -hg npm@latest
npm install -g yarn@$YARN_VERSION bower@$BOWER_VERSION
