{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs
    pkgs.vim
    pkgs.btop
    pkgs.tmux
    pkgs.zsh
  ];

  shellHook = ''
    export SHELL=$(which zsh)
    exec zsh
  '';
}

