{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        bInputs = with pkgs; [
          mold
          clang
          rust-bin.nightly.latest.default
          rust-analyzer
          aoc-cli
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = bInputs;
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
