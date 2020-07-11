let
  sources = import nix/sources.nix;
  pkgs = import sources.nixpkgs {};
in

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo

    rustfmt
    unstable.rust-analyzer
  ];
}
