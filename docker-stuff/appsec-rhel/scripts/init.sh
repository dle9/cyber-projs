#!/bin/bash

mkdir ${APPSEC_USER_HOME}/tmp
mkdir ${APPSEC_USER_HOME}/tools

# install
# && apt-get install -y git build-essential coreutils unzip \
sudo dnf update -y \
&& sudo dnf install -y git wget curl unzip \
&& sudo dnf install -y tar file make gcc yum-utils \
&& sudo dnf install -y openssl-devel bzip2-devel libffi-devel zlib-devel \
&& sudo dnf clean all
