#!/bin/bash

PHP_VERSION=${1:-8.2}

# install
sudo apt-get update -y \
&& sudo apt-get install -y software-properties-common \
&& sudo add-apt-repository -y ppa:ondrej/php \
&& sudo apt-get -y install php$PHP_VERSION \
&& sudo apt-get clean