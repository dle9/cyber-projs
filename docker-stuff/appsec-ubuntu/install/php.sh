#!/bin/bash

PHP_VERSION=${1:-8.2}

# install
apt-get update \
&& apt-get -y install php$PHP_VERSION
