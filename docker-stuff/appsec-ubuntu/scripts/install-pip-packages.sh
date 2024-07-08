#!/bin/bash

# poetry
curl -sSL https://install.python-poetry.org | python3 -

# finish
echo "export PATH=/root/.local/bin:$PATH" >> $HOME/.bashrc
exec /bin/bash
