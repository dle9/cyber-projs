#!/bin/bash

RUBY_VERSION=${1:-3.1.2}

# setup
apt-get update -y 
apt-get install -y libssl-dev libreadline-dev zlib1g-dev autoconf bison \
    libyaml-dev libffi-dev libgdbm-dev libncurses5-dev \ 
    libsqlite3-dev libtool pkg-config

# install rbenv
apt-get install -y rbenv

# install ruby,ruby-build
rbenv install ${RUBY_VERSION}
exec /bin/bash
