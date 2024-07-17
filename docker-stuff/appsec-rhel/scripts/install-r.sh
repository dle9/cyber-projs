#!/bin/bash

R_VERSION=${1:-4.3.3}
INSTALL_URL=https://cran.r-project.org/src/base/R-4/

# setup
sudo dnf update -y \
&& sudo dnf install -y gcc-gfortran xz-devel libcurl-devel \
&& sudo dnf install -y texinfo texlive texlive-collection-latex texlive-collection-fontsrecommended \
&& sudo dnf install -y firefox xpdf

# get the installer
cd $HOME/tmp
sudo curl -Ls $INSTALL_URL/R-$R_VERSION.tar.gz -o r.tar.gz
tar xzf r.tar.gz

# install
cd R-$R_VERSION
./configure --enable-R-shlib --with-x=no --with-readline=no