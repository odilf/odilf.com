{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ nixpkgs, flake-parts, ... }:
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
          pkgs = import nixpkgs { inherit system; };
        in
        {
          formatter = pkgs.nixfmt-rfc-style;
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.nodejs_26
              pkgs.pnpm
              pkgs.svelte-language-server
              pkgs.tailwindcss-language-server
              pkgs.typescript-language-server

              (pkgs.writeShellScriptBin "deploy" ''
                ${pkgs.wrangler}/bin/wrangler pages deploy build/ --project-name "odilf-site" --branch main
              '')
            ];
          };
        };
    };
}
