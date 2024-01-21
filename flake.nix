{
  description = "A development environment for ESP32 with Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        embuild = import ./embuild.nix {
          inherit (pkgs) lib;
          inherit pkgs;
        };
        espup = import ./espup.nix { inherit pkgs; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cmake
            ninja
            dfu-util
            cargo-generate
            cargo-espflash
            cargo-watch
            embuild
            espup

            # esp deps
            flex 
            bison 
            gperf 
            python3
            ccache 
            glibc
            libffi 
            rust-analyzer
            dfu-util
            libusb
          ];
          shellHook = ''
            export EMBUILD=${embuild}
            export ESPUP=${espup}/bin
            export PATH="$ESPUP/bin:$PATH"
            source $ESPUP/esptools.sh
          '';
        };
      });
}

