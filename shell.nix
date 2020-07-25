let
  sources = import nix/sources.nix;
  pkgs = import sources.nixpkgs {};
in

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Rust Dependencies
    rustc
    cargo
    rustfmt
    unstable.rust-analyzer

    # Scala Dependencies
    sbt
  ];
}
