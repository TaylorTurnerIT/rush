{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    codecrafters-cli
    cargo
    rustc
    rustfmt
    clippy
  ];
}
