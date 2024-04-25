" colors
colorscheme dracula
autocmd vimenter * hi Normal guibg=NONE ctermbg=NONE
syntax on " show the colors

" vim QoL
set mouse=a " mouse controls
set tabstop=3
set softtabstop=3
set shiftwidth=3
set number " line numbers
set showcmd
set ruler
" set foldmethod=indent
set clipboard=unnamed " allow yank to host clipboard
set t_Co=256 " use colors

" coding QoL
set nocp
set backspace=indent,eol,start
filetype indent on
inoremap {      {}<Left>
inoremap {<CR>  {<CR>}<Esc>O
inoremap {{     {
inoremap {}     {}
