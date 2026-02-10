let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/41e216c0ca66c83b12ab7a98cc326b5db01db646.tar.gz";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    cargo-llvm-cov
    rustc.llvmPackages.llvm
    rustfmt
    clippy
    rust-analyzer
    sqlite
  ];

  shellHook = ''
    export LLVM_COV=${pkgs.llvm}/bin/llvm-cov
    export LLVM_PROFDATA=${pkgs.llvm}/bin/llvm-profdata
  '';
}
