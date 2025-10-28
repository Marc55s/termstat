{
  description = "A flake for Termstat (package, shell, module, and overlay)";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    let
      overlay = (final: prev: {
        termstat = self.packages.${prev.system}.default;
      });

      homeManagerModule = import ./nix/module.nix;

    in
    (flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        termstat-pkg = pkgs.rustPlatform.buildRustPackage {
          pname = "termstat";
          version = "0.1.0";
          
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

      in
      {
        packages.default = termstat-pkg;

        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer
          ];

          shellHook = ''
            echo "Termstat env loaded! (Package and Shell)"
          '';
        };
      }
    ))
    // {
      overlays.default = overlay;
      homeManagerModules.default = homeManagerModule;
    };
}
