{
  inputs = {
    nixpkgs.url = "github:nixOS/nixpkgs?ref=nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    {
      devShells.default = nixpkgs.legacyPackages.${system}.callPackage ./shell.nix {};
    });
}
