{
  description = "vorpal-sdk";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {
        config,
        pkgs,
        ...
      }: let
        inherit (pkgs) darwin grpcurl just lib openssl pkg-config protobuf rustPlatform;
        inherit (darwin.apple_sdk.frameworks) CoreServices SystemConfiguration Security;
        inherit (rustPlatform) buildRustPackage;
      in {
        packages = {
          sdk-rust = buildRustPackage {
            buildInputs = [openssl] ++ lib.optionals pkgs.stdenv.isDarwin [CoreServices SystemConfiguration Security];
            cargoLock = {
              lockFile = ./rust/Cargo.lock;
              outputHashes = {
                "vorpal-0.1.0" = "sha256-j1pTEN15Mx/9nAC75GAPRJvaoAW8qbped7aJ88qKUKc=";
              };
            };
            nativeBuildInputs = [pkg-config protobuf];
            pname = "vorpal-sdk";
            src = ./rust;
            version = "0.1.0";
          };
        };

        devShells = {
          default = pkgs.mkShell {
            inputsFrom = [config.packages.sdk-rust];
            nativeBuildInputs = [grpcurl just];
          };
        };
      };
    };
}
