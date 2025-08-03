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
              pkgs.tailwindcss_4
              pkgs.bacon
              pkgs.static-web-server
              (pkgs.writeShellScriptBin "serve" ''
                (trap 'kill 0' SIGINT;
                bacon run &
                tailwindcss --input public/app.css --output target/site/static/app.css --watch &
                static-web-server -d target/site/ -a 127.0.0.1 -p 3000 -g trace &
                wait
                )
              '')
            ];

            ODILF_BLOG_PATH = "/Users/odilf/brain/personal/writing";
          };
        };
    };
}
