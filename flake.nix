{
  description = "odilf.com (personal site)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      rec {
        formatter = pkgs.nixfmt-rfc-style;

        devShells.default = pkgs.mkShell rec {
          buildInputs = [
            (pkgs.rust-bin.nightly.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            pkgs.rust-analyzer

            pkgs.jujutsu

            pkgs.vscode-langservers-extracted
            pkgs.leptosfmt
            pkgs.cargo-leptos
            pkgs.trunk
            pkgs.tailwindcss_4
          ];
        };
      }
    );
}
