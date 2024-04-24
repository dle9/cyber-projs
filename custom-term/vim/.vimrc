" colors
colorscheme dracula
syntax on

" vim QoL
set mouse=a
set tabstop=3
set softtabstop=3
set shiftwidth=3
set number
set showcmd
set ruler
set foldmethod=indent
set clipboard=unnamed
set t_Co=256

" coding QoL
set nocp
set backspace=indent,eol,start
filetype indent on
inoremap {      {}<Left>
inoremap {<CR>  {<CR>}<Esc>O
inoremap {{     {
inoremap {}     {}

inoremap [      []<Left>
inoremap [<CR>  [<CR>]<Esc>O
inoremap [[     [
inoremap []     []

inoremap (      ()<Left>
inoremap (<CR>  ()
inoremap ((		 (
inoremap ()		 ()

inoremap "      ""<Left>
inoremap "<CR>  "<CR>"<Esc>O
inoremap ""     "
inoremap ""     ""

inoremap '      ''<Left>
inoremap '<CR>  '<CR>'<Esc>O
inoremap ''     '
inoremap ''     ''
