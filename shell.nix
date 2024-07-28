let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    gcc_multi
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}
