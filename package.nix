{ lib, craneLib }:
let src = craneLib.cleanCargoSource ./.;
    cargoArtifacts = craneLib.buildDepsOnly {
      pname = "bitviz-deps";
      version = "0.1.0";
      inherit src;
      strictDeps = true;
    };
in
craneLib.buildPackage {
  pname = "bitviz";
  cargoExtraArgs = "-p bitviz-cli";
  src = ./.;
  inherit cargoArtifacts;
  inherit (craneLib.crateNameFromCargoToml { src=./bitviz-cli; }) version;
  strictDeps = true;
}
