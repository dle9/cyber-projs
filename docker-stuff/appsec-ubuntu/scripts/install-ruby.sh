#!/bin/bash

RUBY_VERSION=${1:-3.3.3}

# setup
apt-get update -y \
&& apt-get install -y libssl-dev libreadline-dev zlib1g-dev autoconf bison libyaml-dev libffi-dev libgdbm-dev libncurses5-dev libsqlite3-dev libtool pkg-config

# install rbenv
git clone https://github.com/rbenv/rbenv.git ~/.rbenv

# install ruby-build
git clone https://github.com/rbenv/ruby-build.git "$($HOME/.rbenv/bin/rbenv root)"/plugins/ruby-build

# install ruby
$HOME/.rbenv/bin/rbenv init
$HOME/.rbenv/bin/rbenv install $RUBY_VERSION
$HOME/.rbenv/bin/rbenv global $RUBY_VERSION

# finish
exec /bin/bash

# install option 2: faster way to install ruby:
    # does not get latest versions
# apt-get install -y ruby-full
