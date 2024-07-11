#!/bin/bash

# poetry
cd /tmp
curl -sSL https://install.python-poetry.org | python3 -

# finish
echo "export PATH=$PATH:$HOME/.local/bin" >> $HOME/.bashrc
exec /bin/bash
