{
  description = "odilf.com (personal site)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        toolchain = (
          pkgs.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              targets = [
                "wasm32-unknown-unknown"
                # "aarch64-apple-darwin"
                # system
              ];
            }
          )
        );

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        cargo = (pkgs.lib.importTOML ./Cargo.toml).package;

        cargo-leptos' = pkgs.cargo-leptos;
        cargo-leptos =
          let
            inherit (pkgs.darwin.apple_sdk.frameworks)
              CoreServices
              SystemConfiguration
              Security
              ;
            inherit (pkgs.lib) optionals;
            inherit (pkgs.stdenv.hostPlatform) isDarwin;
          in
          (pkgs.rustPlatform.buildRustPackage.override { stdenv = pkgs.gcc13Stdenv; }) rec {
            pname = "cargo-leptos";
            version = "0.2.32";

            src = pkgs.fetchFromGitHub {
              owner = "leptos-rs";
              repo = "cargo-leptos";
              rev = "v${version}";
              hash = "sha256-sRfIRPs0hhFjMK9KQ4AUDSGTu1U8QIpVxsVhsWjY4fs=";
            };

            useFetchCargoVendor = true;
            cargoHash = "sha256-zSZxgC+S0ovknSgNhRa0GvHjVp97njgKHjOCz8cB5aE=";

            nativeBuildInputs = [
              pkgs.gcc13Stdenv.cc.cc.lib
              pkgs.perl
            ];

            buildInputs =
              [
                pkgs.pkg-config
                pkgs.openssl
              ]
              ++ optionals isDarwin [
                SystemConfiguration
                Security
                CoreServices
              ];

            # https://github.com/leptos-rs/cargo-leptos#dependencies
            buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to install missing dependencies on its own otherwise
            doCheck = false; # Check phase tries to query crates.io
          };
      in
      {
        packages.default = naersk'.buildPackage {
          src = ./.;

          # cargo build --package=odilf-site --lib --target-dir=/Users/odilf/code/odilf.com/target/front --target=wasm32-unknown-unknown --no-default-features --features=hydrate --profile=wasm-release
          cargoBuild = _: ''
            cargo leptos --version
            cargo leptos build --release -vv >> $cargo_build_output_json
          '';

          postInstall = "TODO";
          nativeBuildInputs = [
            cargo-leptos
            pkgs.binaryen

          ];

          buildInputs = [
            pkgs.tailwindcss_4
            pkgs.trunk
            pkgs.pkg-config
            pkgs.openssl
          ];

          # TODO: Is this necessary?
          # From https://github.com/marcuswhybrow/leptos-issue-3288/blob/0a06b2e1e1d6a9c1051ec9666684ea8a4c3f7262/flake.nix#L36
          #
          # Fixes production build
          #
          # Leptos' WASM URL is a function of Leptos' output name. This is set in
          # Cargo.toml, which works fine whilst using the `cargo-leptos` dev server
          # during development. Build for production using `nix build` or `nix run`
          # will (for some reason I don't understand) fail to pick up this variable
          # from Cargo.toml, despite picking up other variables from Cargo.toml
          # such as the server port number.
          #
          # Naersk.buildPackage (I believe) sets up all unknown attributes as
          # environment variables for the build process, which in this case tells
          # Leptos which output name to look for.
          LEPTOS_OUTPUT_NAME = cargo.metadata.leptos.output-name;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-analyzer
            toolchain

            pkgs.jujutsu

            pkgs.vscode-langservers-extracted
            pkgs.leptosfmt
            pkgs.cargo-leptos
            pkgs.trunk
            pkgs.tailwindcss_4
            pkgs.binaryen
            pkgs.nixd
            pkgs.nil
          ];

          ODILF_BLOG_PATH = "/Users/odilf/brain/personal/writing";
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
