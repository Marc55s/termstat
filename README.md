# 💻 Termstat - A tracker for Commands
A complete local command tracker, no Cloud involved. 

! Still in development !

## How it works
1. A Shell prehook logs all commands into a logfile
2. If the sync command is called the logfile entries are inserted into a Sqlite database

## Installation
### Shell prehook
Add the following to your .zshrc
~~~sh
eval "$(termstat init --shell-type zsh)"
~~~

- Nix
- Cargo
- From Source

## Currently Supported Shells
- [x] Zsh
- [ ] Bash
- [ ] Fish
- [ ] ...

## Commands
- [x] sync
- [x] stats --daily, --weekly, --monthly
- [x] init --shell-type [SHELL]
- [ ] clean

## To Do
- [x] Switch from flags to subcommands
- [ ] Systemd service for syncing automatically
- [ ] Support multiple Shelltypes
- [ ] Display statistics in a fancy way with a TUI-Lib
- [ ] Write Installation Manual
- [ ] Nix Packaging / Module
- [ ] Publish to Crates.io
- [x] List available Commands
- [ ] Far future: Support for multiple databases / Syncing across devices

## Queries
- [x] Most used command of the day|week|month
- [ ] Commands sorted by length
- [ ] Common command pipes
- [ ] Top exectued binaries
