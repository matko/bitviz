{pkgs, mkShell}:
mkShell {
  buildInputs = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "rust-analyzer" ];
    })
  ];
}
