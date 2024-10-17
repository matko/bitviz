{
  inputs = {
    nixpkgs.url = "github:nixOS/nixpkgs?ref=nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = (import nixpkgs) { inherit system; overlays = [(import rust-overlay)]; }; in
      {
        devShells.default = pkgs.callPackage ./shell.nix {};
      });
}
