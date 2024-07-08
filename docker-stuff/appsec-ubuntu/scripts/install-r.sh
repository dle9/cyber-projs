#!/bin/bash

R_VERSION=${1:-4.3.3}

# install it 
apt-get -y update \
&& apt-get install -y r-base