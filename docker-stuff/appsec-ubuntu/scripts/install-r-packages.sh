#!/bin/bash

PACKRAT_VERSION=${1:-0.9.0}

# install packrat
sudo R -e "install.packages('packrat')"