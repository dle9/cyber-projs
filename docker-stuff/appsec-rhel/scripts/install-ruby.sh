# #!/bin/bash

# RUBY_VERSION=${1:-3.2.4}

# # setup
# sudo dnf update -y

# # install rbenv
# git clone https://github.com/rbenv/rbenv.git ~/.rbenv

# # install ruby-build
# git clone https://github.com/rbenv/ruby-build.git "$($HOME/.rbenv/bin/rbenv root)"/plugins/ruby-build

# # install ruby
# $HOME/.rbenv/bin/rbenv init
# RUBY_CONFIGURE_OPTS="--with-openssl-dir=/usr" $HOME/.rbenv/bin/rbenv install $RUBY_VERSION
# $HOME/.rbenv/bin/rbenv global $RUBY_VERSION

# # finish
# echo 'eval "$(rbenv init -)"' >> ~/.bashrc




# sudo dnf install -y libffi-devel openssl-devel readline zlib-devel