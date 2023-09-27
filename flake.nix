{
  description = "Media Subscriber";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        SERVER_SECRET =
          "2a4855f2c950a0e6167d80059e657647b70231529bec3015a56188a387f956e8cdf6277e06f5a4c335e9782971b984e3feb600f4a19acf536240f51a16560575";
        SERVER_PORT = "7070";
        RUST_LOG = "debug";
        commonInputs = with pkgs;
          [ openssl pkg-config rust-bin.nightly."2023-07-04".default nodejs_20 ]
          ++ (if stdenv.isDarwin then
            with darwin.apple_sdk.frameworks; [
              IOKit
              Security
              CoreServices
              SystemConfiguration
            ]
          else
            [ ]);
      in with pkgs; {
        devShells = {
          default = mkShell {
            inherit SERVER_SECRET SERVER_PORT RUST_LOG;
            buildInputs = commonInputs;
          };
          backend = mkShell {
            inherit SERVER_SECRET SERVER_PORT RUST_LOG;
            buildInputs = commonInputs ++ [
              cargo-limit
              rust-analyzer
              nodePackages.svelte-language-server
            ];
          };
          frontend = mkShell {
            inherit SERVER_SECRET SERVER_PORT RUST_LOG;
            buildInputs = commonInputs
              ++ [ nodePackages.svelte-language-server ];
          };
        };
      });
}
