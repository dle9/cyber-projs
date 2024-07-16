#!/bin/bash

PHP_VERSION=${1:-8.2}

# get extra repo containing old php
sudo dnf install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm -y \
&& sudo dnf -y install http://rpms.remirepo.net/enterprise/remi-release-8.rpm -y

# install the php
sudo dnf update -y \
&& sudo dnf module -y enable php:remi-8.2 \
&& sudo dnf install -y php