#!/bin/bash

YARN_VERSION=${1:-1.22.10}
BOWER_VERSION=${2:-1.8.10}

# install
npm install -hg npm@latest
npm install -g yarn@$YARN_VERSION bower@$BOWER_VERSION
