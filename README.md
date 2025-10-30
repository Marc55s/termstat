# 💻 Termstat - A tracker for Commands !!!(Still in development)!!!
A complete local command tracker, no Cloud involved. Feel free to try it, but keep in mind, that some things may break in following updates.

## How it works
1. A Shell prehook logs all commands into a logfile
2. If the sync command is called the logfile entries are inserted into a Sqlite database

## Installation
- [ ] Nix
    - [ ] Systemd
- [ ] Cargo
- [ ] From Source

### Shell prehook
Add the following to your .zshrc
~~~sh
eval "$(termstat init --shell-type zsh)"
~~~
### Nix
```Nix
Install nix and configure terstat via Homemanager
# flake.nix

# add input
inputs = {
    termstat = {
        url = "github:marc55s/termstat";
        inputs.nixpkgs.follows = "nixpkgs-unstable";
    };
}
# add overlay to your outputs
outputs = inputs@{self, nixpkgs, termstat ... }:
    pkgs = import nixpkgs {
        inherit system;
        overlays = [ inputs.termstat.overlays.default];
    }

# home.nix
{termstat, ...}:
{
    programs.termstat = {
        enable = true;
        enableZshIntegration = true;

        # Not supported yet
        # enableBashIntegration = true;
        # enableFishIntegration = true;
        # enableIonIntegration = true;
        # enableNushellIntegration = true;
    };

    # add the modules to your imports
    imports = [termstat.homeManagerModules.default];
}

```
## Usage
After the initalization the following commands are available:
> Help
![](/screenshots/help_command.png)

> Sync with database
![](/screenshots/sync_command.png)
> Statistics
![](/screenshots/stats_daily.png)

For the statistics these other commandflags are available:
```Bash
termstat stats # default to termstat stats --daily/-d
termstat stats --weekly/-w
termstat stats --monthly/-m

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
- [x] List available Commands
- [ ] Nix Packaging / Module
- [ ] Systemd service for syncing automatically
- [ ] Support multiple Shelltypes
- [ ] Display statistics in a fancy way with a TUI-Lib
- [ ] Write Installation Manual
- [ ] Publish to Crates.io
- [ ] Far future: Support for multiple databases / Syncing across devices

## Queries
- [x] Most used command of the day|week|month
- [ ] Commands sorted by length
- [ ] Common command pipes
- [ ] Top exectued binaries
