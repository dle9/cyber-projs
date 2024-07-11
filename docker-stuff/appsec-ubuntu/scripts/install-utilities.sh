#!/bin/bash

# install
# && apt-get install -y git build-essential coreutils unzip \
sudo apt-get update -y \
&& sudo apt-get install -y git wget curl unzip \
&& sudo apt-get install -y tar file \
&& sudo apt-get clean
