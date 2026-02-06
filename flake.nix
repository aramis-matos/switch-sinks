{
  description = "A very basic flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) { inherit system overlays; };
      in
      {
        packages = rec {
          switch-sink = pkgs.rustPlatform.buildRustPackage {
            pname = "switch-sink";
            version = "1.0.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildInputs = with pkgs; [
              rust-analyzer
            ];

            nativeBuildInputs = with pkgs; [
              rust-analyzer
              pulseaudio
              bash

            ];

          };
          default = switch-sink;
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
          buildInputs = with pkgs; [
            cargo
            cargo-udeps
            clippy
            rust-analyzer
            rustfmt
            rustc
            code
          ];
        };
      }
    );

}
