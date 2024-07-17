# #!/bin/bash

RUBY_VERSION=${1:-3.1.6}

# setup
sudo dnf update -y \
&& sudo dnf install gcc make redhat-rpm-config -y \

# install rbenv
git clone https://github.com/rbenv/rbenv.git ~/.rbenv

# install ruby-build
git clone https://github.com/rbenv/ruby-build.git "$($HOME/.rbenv/bin/rbenv root)"/plugins/ruby-build

# install ruby
$HOME/.rbenv/bin/rbenv init
$HOME/.rbenv/bin/rbenv install $RUBY_VERSION
$HOME/.rbenv/bin/rbenv global $RUBY_VERSION

# finish        
echo 'eval "$(rbenv init -)"' >> ~/.bashrc
# export RUBY_HOME=$RUBY_VERSION