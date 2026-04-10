{
  description = "workingon - A Rust CLI for tracking what you're working on";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/83e29f2b8791f6dec20804382fcd9a666d744c07";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = { };
          overlays = [ ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            cargo-llvm-cov
            clippy
            hooky
            nixpkgs-fmt
            rust-analyzer
            rustc
            rustc.llvmPackages.llvm
            rustfmt
            sqlite
          ];

          shellHook = ''
            export LLVM_COV=${pkgs.llvm}/bin/llvm-cov
            export LLVM_PROFDATA=${pkgs.llvm}/bin/llvm-profdata
          '';
        };
      }
    );
}
