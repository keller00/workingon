let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/83e29f2b8791f6dec20804382fcd9a666d744c07.tar.gz";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
  packages = with pkgs; [
    hooky
    cargo
    cargo-llvm-cov
    rustc
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
