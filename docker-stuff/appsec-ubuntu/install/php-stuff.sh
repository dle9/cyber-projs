#!/bin/bash

COMPOSER_VERSION=${1:-2.6.6}

# get installer
php -r "copy('https://getcomposer.org/installer', 'composer-setup.php');"

# verify
php -r "if (hash_file('sha384', 'composer-setup.php') === 'dac665fdc30fdd8ec78b38b9800061b4150413ff2e3b6f88543c636f7cd84f6db9189d43a81e5503cda447da73c7e5b6') { echo 'Installer verified'; } else { echo 'Installer corrupt'; unlink('composer-setup.php'); } echo PHP_EOL;"

# setup
php composer-setup.php --version=$COMPOSER_VERSION

# make it global
mv composer.phar /usr/local/bin/composer

# clean
php -r "unlink('composer-setup.php');"
