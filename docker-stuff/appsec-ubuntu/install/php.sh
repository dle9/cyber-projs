#!/bin/bash

PHP_VERSION=${1:-8.2}

# install
apt-get update -y \
&& apt-get install -y software-properties-common \
&& add-apt-repository -y ppa:ondrej/php \
&& apt-get -y install php$PHP_VERSION

