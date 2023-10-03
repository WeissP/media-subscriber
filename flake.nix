{
  description = "getting started example";

  inputs = {
    devshell.url = "github:numtide/devshell";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, flake-utils, devshell, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default (import rust-overlay) ];
        };
        basic = {
          imports = map pkgs.devshell.importTOML [
            ./env_config/db.toml
            ./env_config/server.toml
          ];
          packages = with pkgs;
            [
              rust-bin.nightly."2023-07-04".default
              openssl
              openssl.dev
              pkg-config
            ] ++ (if stdenv.isDarwin then
              with darwin.apple_sdk.frameworks; [
                IOKit
                Security
                CoreServices
                SystemConfiguration
              ]
            else
              [ ]);
          env = [
            {
              name = "PKG_CONFIG_PATH";
              value = "${pkgs.openssl.dev}/lib/pkgconfig";
            }
            {
              name = "OPENSSL_DIR";
              value = pkgs.openssl.dev;
            }
          ];
        };
        extraImports = files:
          basic // {
            imports = basic.imports ++ map pkgs.devshell.importTOML files;
          };
      in {
        devShells = {
          default = pkgs.devshell.mkShell basic;
          backend =
            pkgs.devshell.mkShell (extraImports [ ./env_config/backend.toml ]);
          frontend =
            pkgs.devshell.mkShell (extraImports [ ./env_config/frontend.toml ]);
          ci = pkgs.devshell.mkShell (extraImports [ ./env_config/ci.toml ]);
        };
      });
}
