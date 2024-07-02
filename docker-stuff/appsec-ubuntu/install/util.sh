#!/bin/bash

# install
apt-get update \
&& apt-get install -y wget curl coreutils unzip \
&& apt-get install -y git sqlite3 build-essential \
&& apt-get clean
