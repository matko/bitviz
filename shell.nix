{pkgs, mkShell}:
mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
  ];
}
