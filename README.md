# jer
## Jericho's Editing Repository

A personal fun project to build my own text editor combining my favorite features of neovim, helix/kakoune and emacs in Rust.

### Main Idea's / Goals
- Editing navigation similar to Emacs and Bash (advanced "insert mode" editing is just OP imo)
- Vim-Easymotion/Neovim hop like movement in addition to Emacs basics (ctrl-n, ctrl-p, alt-f, alt-b) 
- Mostly non-modal editing
- Going to try to be batteries included similar to Helix (no plugins, lsp just works... hopefully)
- Try to stick to Unix Philosophy, similar to Kakoune. Pipe most non-trivial edits to linux coreutils
- Keep/Augment many vim key commands like ctrl-u/ctrl-d, maybe hjkl for non-editing navigation (insane right?)

### Pie in the sky ideas / Fun challenges (Non-unix philosophy stuff)
- Automatic CSV / Json recognition/editing
- Use ML/NLP for syntax highlighting/file recognition
- Live regex highlighting like RegExr
- Easy Rest API generator/HTTP request builder/editor
