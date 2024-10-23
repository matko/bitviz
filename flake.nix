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
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = (import nixpkgs) { inherit system; overlays = [(import rust-overlay)
                                                                (final: prev: {
                                                                  craneLib = (crane.mkLib prev).overrideToolchain final.rust-bin.stable.latest.minimal;
                                                                })
                                                               ];
                                  }; in
      {
        packages.default = pkgs.callPackage ./package.nix {};
        devShells.default = pkgs.callPackage ./shell.nix {};
      });
}
