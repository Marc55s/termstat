{ config, pkgs, pkgs-unstable, lib, ... }:
let
  inherit (lib) mkIf mkOption types;

  cfg = config.programs.termstat;
  syncCfg = cfg.systemd;

  # initFish = if cfg.enableInteractive then "interactiveShellInit" else "shellInitLast";
in {
  meta.maintainers = [ ];

  options.programs.termstat = {
    enable = lib.mkEnableOption "termstat";

    package = lib.mkPackageOption pkgs "termstat" { };

    enableBashIntegration =
      lib.hm.shell.mkBashIntegrationOption { inherit config; };

    enableFishIntegration =
      lib.hm.shell.mkFishIntegrationOption { inherit config; };

    enableIonIntegration =
      lib.hm.shell.mkIonIntegrationOption { inherit config; };

    enableNushellIntegration =
      lib.hm.shell.mkNushellIntegrationOption { inherit config; };

    enableZshIntegration =
      lib.hm.shell.mkZshIntegrationOption { inherit config; };

    enableInteractive = mkOption {
      type = types.bool;
      default = true;
      description = ''
        Only enable termstat when the shell is interactive. This option is only
        valid for the Fish shell.

        Some plugins require this to be set to `false` to function correctly.
      '';
    };

    systemd = {
      enable = lib.mkEnableOption "periodic termstat sync timer";

      onCalendar = mkOption {
        type = types.str;
        default = "daily";
        description = "Systemd OnCalendar expression for when to run the sync.";
        example = "hourly";
      };

      command = mkOption {
        type = types.str;
        default = "sync";
        description = "The termstat subcommand to run for syncing.";
      };
    };

  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    programs.bash.initExtra = mkIf cfg.enableBashIntegration ''
      if [[ $TERM != "dumb" ]]; then
        eval "$(${lib.getExe cfg.package} init --shell-type bash)"
      fi
    '';

    programs.zsh.initContent = mkIf cfg.enableZshIntegration ''
      if [[ $TERM != "dumb" ]]; then
        eval "$(${lib.getExe cfg.package} init --shell-type zsh)"
      fi
    '';

    # programs.fish.${initFish} = mkIf cfg.enableFishIntegration ''
    #   if test "$TERM" != "dumb"
    #     ${lib.getExe cfg.package} init fish | source
    #     ${lib.optionalString cfg.enableTransience "enable_transience"}
    #   end
    # '';

    programs.ion.initExtra = mkIf cfg.enableIonIntegration ''
      if test $TERM != "dumb"
        eval $(${lib.getExe cfg.package} init --shell-type ion)
      end
    '';

    programs.nushell = mkIf cfg.enableNushellIntegration {
      # Unfortunately nushell doesn't allow conditionally sourcing nor
      # conditionally setting (global) environment variables, which is why the
      # check for terminal compatibility (as seen above for the other shells) is
      # not done here.
      extraConfig = ''
        use ${
          pkgs.runCommand "termstat-nushell-config.nu" { } ''
            ${lib.getExe cfg.package} init --shell-type nu >> "$out"
          ''
        }
      '';
    };

    # Service Unit-File
    systemd.user.services.termstat-sync = mkIf syncCfg.enable {
      Unit = { Description = "Termstat Sync Command"; };
      Service = {
        Type = "oneshot";
        ExecStart = "${lib.getExe cfg.package} ${syncCfg.command}";
      };
    };

    # Timer Unit-File
    systemd.user.timers.termstat-sync = mkIf syncCfg.enable {
      Unit = { Description = "Run termstat sync command periodically"; };
      Timer = {
        OnCalendar = syncCfg.onCalendar;
        Persistent = true; # Run on next boot if a run was missed
      };
      Install = {
        WantedBy = [ "timers.target" ];
      };
    };
  };
}
