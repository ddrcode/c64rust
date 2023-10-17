{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ flake-parts
    , nixpkgs
    , treefmt-nix
    , rust-overlay
    , ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        treefmt-nix.flakeModule
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { self', system, config, pkgs, ... }:
        let
          lib = pkgs.lib;
          stdenv = pkgs.stdenv;

          rustVersion = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rustfmt" "rust-analyzer" ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustVersion;
            rustc = rustVersion;
          };

          nativeBuildInputs = [
          ] ++ lib.optionals stdenv.isLinux [
            pkgs.pkg-config
          ];

          crateInfo = builtins.fromTOML (builtins.readFile ./Cargo.toml);

          projectCrate = rustPlatform.buildRustPackage {
            inherit (crateInfo.package) name description;

            crates = ./.;

            nativeBuildInputs = nativeBuildInputs;

            cargoLock.lockFile = ./Cargo.lock;
          };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };

          packages = rec {
            default = c64emu;
            c64emu = projectCrate;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = nativeBuildInputs;

            buildInputs = [
              # projectCrate

              pkgs.cargo-watch
              pkgs.cargo-machete
              pkgs.rust-analyzer
              pkgs.clippy

              rustVersion

              pkgs.treefmt
              pkgs.acme
              # pkgs.cc65
            ];

            shellHook = ''
              export INT_FILTER_LEVEL="off"
              export EXT_FILTER_LEVEL="debug"
              export RUST_LOG="info"
              export CURSIVE_LOG="off"
              export RUST_BACKTRACE=1
            '';

          };

          treefmt.config = {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.rustfmt.package = rustVersion;
            programs.prettier.enable = true;
          };

          overlayAttrs = {
            inherit (self'.packages) c64emu;
          };
        };
    };
}
