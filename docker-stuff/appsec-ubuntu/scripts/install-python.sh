#!/bin/bash

# remove python 3.12
apt remove python3.12 -y \
&& apt purge python3.12 -y \
&& apt autoremove -y

# install python 3.9
apt-get update -y \
&& apt-get install software-properties-common -y \
&& add-apt-repository ppa:deadsnakes/ppa -y \
&& apt-get install -y python3.9

# remove it again
apt remove python3.12 -y \
&& apt purge python3.12 -y \
&& apt autoremove -y

# link it up
ln -s /usr/bin/python3.9 /usr/bin/python3

# install pip and pipenv
apt-get install -y python3.9-distutils
curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
python3 get-pip.py 
pip install pipenv

# clean
rm get-pip.py
exec /bin/bash
