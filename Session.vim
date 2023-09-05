let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
let NvimTreeSetup =  1 
let Tabline_session_data = "[{\"show_all_buffers\": true, \"name\": \"1\", \"allowed_buffers\": []}]"
let NvimTreeRequired =  1 
silent only
silent tabonly
cd ~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +41 gramma.g
badd +131 src/parser.rs
badd +4 grammaV2.g
badd +7 src/main.rs
badd +1 ~/Desktop/Projekte/PATRISPREDICTUM/Projekte/Schule/SortTest/src/GUI.c
badd +40 term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//26772:C:/Windows/system32/cmd.exe
badd +20 src/nda_collapse.rs
badd +0 term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//5472:C:/Windows/system32/cmd.exe
badd +6 calc.g
argglobal
%argdel
edit src/nda_collapse.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd _ | wincmd |
split
1wincmd k
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
exe 'vert 1resize ' . ((&columns * 146 + 146) / 293)
exe '2resize ' . ((&lines * 40 + 41) / 83)
exe 'vert 2resize ' . ((&columns * 146 + 146) / 293)
exe '3resize ' . ((&lines * 39 + 41) / 83)
exe 'vert 3resize ' . ((&columns * 146 + 146) / 293)
argglobal
balt src/main.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 87 - ((58 * winheight(0) + 40) / 80)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 87
normal! 076|
wincmd w
argglobal
if bufexists(fnamemodify("calc.g", ":p")) | buffer calc.g | else | edit calc.g | endif
if &buftype ==# 'terminal'
  silent file calc.g
endif
balt src/main.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 6 - ((5 * winheight(0) + 20) / 40)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 6
normal! 010|
wincmd w
argglobal
if bufexists(fnamemodify("term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//5472:C:/Windows/system32/cmd.exe", ":p")) | buffer term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//5472:C:/Windows/system32/cmd.exe | else | edit term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//5472:C:/Windows/system32/cmd.exe | endif
if &buftype ==# 'terminal'
  silent file term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//5472:C:/Windows/system32/cmd.exe
endif
balt term://~/Desktop/Projekte/PATRISPREDICTUM/Projekte/G//26772:C:/Windows/system32/cmd.exe
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 5615 - ((38 * winheight(0) + 19) / 39)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5615
normal! 058|
wincmd w
exe 'vert 1resize ' . ((&columns * 146 + 146) / 293)
exe '2resize ' . ((&lines * 40 + 41) / 83)
exe 'vert 2resize ' . ((&columns * 146 + 146) / 293)
exe '3resize ' . ((&lines * 39 + 41) / 83)
exe 'vert 3resize ' . ((&columns * 146 + 146) / 293)
tabnext 1
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
set hlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
