#!/bin/bash

# RUBY_VERSION=${1:-3.3.3}

# setup
sudo apt-get update -y \
&& sudo apt-get install -y rbenv \
&& sudo apt-get clean





# get latest versions

# # install rbenv
# git clone https://github.com/rbenv/rbenv.git ~/.rbenv

# # install ruby-build
# git clone https://github.com/rbenv/ruby-build.git "$($HOME/.rbenv/bin/rbenv root)"/plugins/ruby-build

# # install ruby
# $HOME/.rbenv/bin/rbenv init
# $HOME/.rbenv/bin/rbenv install $RUBY_VERSION
# $HOME/.rbenv/bin/rbenv global $RUBY_VERSION

# # finish
# exec /bin/bash
