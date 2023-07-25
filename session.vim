let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Coding/WebGPU/learn-wgpu
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 src/challenge_shader.wgsl
badd +1 src/shader.wgsl
badd +33 src/main.rs
badd +2 src/pipelines.rs
badd +1 src/state.rs
badd +0 ./
badd +0 NERD_tree_2
badd +0 NERD_tree_3
argglobal
%argdel
$argadd ./
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/challenge_shader.wgsl
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 94 + 95) / 191)
exe 'vert 2resize ' . ((&columns * 96 + 95) / 191)
argglobal
balt src/shader.wgsl
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=20
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("src/shader.wgsl", ":p")) | buffer src/shader.wgsl | else | edit src/shader.wgsl | endif
if &buftype ==# 'terminal'
  silent file src/shader.wgsl
endif
balt src/challenge_shader.wgsl
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=20
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
exe 'vert 1resize ' . ((&columns * 94 + 95) / 191)
exe 'vert 2resize ' . ((&columns * 96 + 95) / 191)
tabnext
edit src/state.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 81 + 95) / 191)
exe 'vert 2resize ' . ((&columns * 109 + 95) / 191)
argglobal
balt src/main.rs
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=20
setlocal fml=1
setlocal fdn=20
setlocal fen
17
normal! zo
18
normal! zo
33
normal! zo
42
normal! zo
99
normal! zo
109
normal! zo
110
normal! zo
136
normal! zo
151
normal! zo
153
normal! zo
let s:l = 88 - ((33 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 88
let s:c = 13 - ((9 * winwidth(0) + 40) / 81)
if s:c > 0
  exe 'normal! ' . s:c . '|zs' . 13 . '|'
else
  normal! 013|
endif
wincmd w
argglobal
if bufexists(fnamemodify("src/pipelines.rs", ":p")) | buffer src/pipelines.rs | else | edit src/pipelines.rs | endif
if &buftype ==# 'terminal'
  silent file src/pipelines.rs
endif
balt src/main.rs
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=20
setlocal fml=1
setlocal fdn=20
setlocal fen
1
normal! zo
14
normal! zo
let s:l = 21 - ((20 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 21
normal! 010|
wincmd w
exe 'vert 1resize ' . ((&columns * 81 + 95) / 191)
exe 'vert 2resize ' . ((&columns * 109 + 95) / 191)
tabnext 1
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
