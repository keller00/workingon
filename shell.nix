let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/41e216c0ca66c83b12ab7a98cc326b5db01db646.tar.gz";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    sqlite
  ];
}
