{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  name = "spider";

  buildInputs = with pkgs; [
    cargo
    rustc
  ];

  shellHook = ''
    export AWS_PROFILE="jsgeeks"
    export AWS_EB_PROFILE="jsgeeks"
  '';
}

