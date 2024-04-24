# To the extent possible under law, the author(s) have dedicated all 
# copyright and related and neighboring rights to this software to the 
# public domain worldwide. This software is distributed without any warranty. 
# You should have received a copy of the CC0 Public Domain Dedication along 
# with this software. 
# If not, see <https://creativecommons.org/publicdomain/zero/1.0/>. 

# ~/.bashrc: executed by bash(1) for interactive shells.

# The copy in your home directory (~/.bashrc) is yours, please
# feel free to customise it to create a shell
# environment to your liking.  If you feel a change
# would be benifitial to all, please feel free to send
# a patch to the msys2 mailing list.
# User dependent .bashrc file


export PS1="\n\[$(tput setaf 124)\][\[\e[4;38;5;47m\]$(tput setaf 240)\]\h\[$(tput setaf 0)\]\[$(tput sgr0)\]\[$(tput setaf 124)] $(tput setaf 124)\]\w \[$(tput sgr0)\]\n\[$(tput setaf 240)\]>\[$(tput setaf 255)\] "

# If not running interactively, don't do anything
[[ "$-" != *i* ]] && return

alias ls='ls --color=auto'
alias lsa='ls -a --color=auto'
alias lab='cd /c/Users/deric/dleer/cybersecurity/lab'
alias dl='cd /c/Users/deric/Downloads'
alias ~='cd ~'
alias rst='exec /usr/bin/bash'

alias gs='git status'
alias ga='git add'
alias gc='git commit -m'
alias gps='git push'
alias gpl='git pull'

HISTCONTROL=ignoreboth:erasedups

GREP_COLORS='sl=49;39:cx=49;39:mt=49;31;1:fn=49;32:ln=49;33:bn=49;33:se=1;36'
LS_COLORS='di=1;48;5;238:ln=1;30;47:so=30;45:pi=30;45:ex=4;38;5;124:bd=30;46:cd=30;46:su=30'
LS_COLORS="${LS_COLORS}:*.elf=1;31:bd=30;46:cd=30;46:su=30"

export GREP_COLORS LS_COLORS LSCOLORS

export PATH=$PATH:/c/Users/deric/.cargo/bin:/mingw64/bin:/c/Users/deric/AppData/Local/Programs/Microsoft\ VS\ Code/bin:/c/Lab/globals
