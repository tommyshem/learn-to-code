" save file in ~/.config/nvim/init.vim
" run :PlugInstall
" run :
" Website plugins
" autocomplete https://github.com/neoclide/coc.nvim
" install rls https://github.com/neoclide/coc-rls
" install snippets https://github.com/neoclide/coc-snippets
" install git https://github.com/neoclide/coc-git
" :CocInstall coc-git
" :CocInstall coc-rls
" :CocInstall coc-snippets

call plug#begin('~/.vim/plugged')
  
  Plug 'joshdick/onedark.vim'                               " Theme for editor
  Plug 'scrooloose/nerdtree', {'on': 'NERDTreeToggle' }     " File tree
  Plug 'bling/vim-airline'                                  " better status line
  Plug 'jiangmiao/auto-pairs'                               " auto put brackets in editor
  Plug 'luochen1990/rainbow'                                " Rainbow brackets highlighting
  "Plug 'sheerun/vim-polyglot'  
  Plug 'neoclide/coc.nvim', {'do': './install.sh nightly'}  " AutoComplete system
  Plug 'ervandew/supertab'                                  " Supertab 
  Plug 'rust-lang/rust.vim'                                 " Rust plugin
  
  call plug#end()
  

" Neovim setup
set autoindent       " Autoindent if possible
set cmdheight=2      " Command height always 2
set nowrap           " do not wrap text
set signcolumn=yes   " Always draw sign column
set mouse=a          " Enable mouse

" Set rust filetype
autocmd BufReadPost *.rs setlocal filetype=rust
  
" Setup rainbow plugin
let g:rainbow_active = 1  " set to 0 to disabple plugin

" Setup airline to display neoclide coc errors
" if you want to disable auto detect, comment out those two lines
let g:airline#extensions#disable_rtp_load = 1
let g:airline_extensions = ['branch', 'hunks', 'coc']
let g:airline_section_error = '%{airline#util#wrap(airline#extensions#coc#get_error(),0)}'
let g:airline_section_warning = '%{airline#util#wrap(airline#extensions#coc#get_warning(),0)}'
  
"Map tab key for auto complete on neoclide
inoremap <silent><expr> <TAB>
        \ pumvisible() ? coc#_select_confirm() :
        \ coc#expandableOrJumpable() ? "\<C-r>=coc#rpc#request('doKeymap', ['snippets-expand-jump',''])\<CR>" :
        \ <SID>check_back_space() ? "\<TAB>" :
        \ coc#refresh()
  
function! s:check_back_space() abort
let col = col('.') - 1
    return !col || getline('.')[col - 1]  =~# '\s'
  endfunction
  
let g:coc_snippet_next = '<tab>'
  
" Use `:Format` to format current buffer
command! -nargs=0 Format :call CocAction('format')
