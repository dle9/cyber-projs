#!/bin/bash

# install python 3.9
sudo dnf update -y \
&& sudo dnf install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm -y \

&& sudo dnf install software-properties-common -y \
&& sudo add-apt-repository ppa:deadsnakes/ppa -y \
&& sudo dnf install -y python3.9

# force link 3.9 to python3 command
sudo ln -sf /usr/bin/python3.9 /usr/bin/python3

# install pip for 3.9
cd $HOME/tmp
sudo dnf install -y python3.9-distutils
sudo curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
python3 get-pip.py 

sudo rm get-pip.py