#!/bin/bash

# install
apt-get update -y \
&& apt-get install -y wget curl coreutils unzip file \
&& apt-get install -y git sqlite3 build-essential \
&& apt-get install -y sudo git vim nano \
&& apt-get clean
