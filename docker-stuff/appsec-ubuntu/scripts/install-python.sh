#!/bin/bash

# install python 3.9
sudo apt-get update -y \
&& sudo apt-get install -y pip \
&& sudo apt-get install software-properties-common -y \
&& sudo add-apt-repository ppa:deadsnakes/ppa -y \
&& sudo apt-get install -y python3.9

# remove python 3.12
sudo apt remove python3.12 -y \
&& sudo apt purge python3.12 -y \
&& sudo apt autoremove -y

# link it up
sudo ln -s /usr/bin/python3.9 /usr/bin/python3

# clean
rm get-pip.py
exec /bin/bash
