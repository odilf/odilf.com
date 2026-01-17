{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      nixpkgs,
      rust-overlay,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { system, ... }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
        in
        {
          formatter = pkgs.nixfmt-rfc-style;
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.rust-bin.beta.latest.default
              pkgs.rust-analyzer
              pkgs.just
              pkgs.openssl
              pkgs.pkg-config

              pkgs.watchexec
              pkgs.live-server

              pkgs.tailwindcss_4
              pkgs.static-web-server
              pkgs.wrangler
              pkgs.imagemagick

              (pkgs.writeShellScriptBin "deploy" ''
                cargo run --release && wrangler pages deploy target/release/site/ --project-name "odilf-site" --branch main
              '')
            ];
          };
        };
    };
}
