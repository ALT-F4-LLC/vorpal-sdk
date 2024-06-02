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
        inherit (pkgs) grpcurl just protobuf rustPlatform;
        inherit (rustPlatform) buildRustPackage;
      in {
        packages = {
          example-rust = buildRustPackage {
            cargoLock = {
              lockFile = ./example/rust/Cargo.lock;
              outputHashes = {
                "vorpal-0.1.0" = "sha256-yps0MqnBtA1xF+0ci8V7GDFxvoxfd3ufKMlHkW9bQSM=";
                "vorpal-sdk-0.1.0" = "sha256-U/8lEM2B7MGCckams+uz0uozv4Yq499gVoaIAK8jV7o=";
              };
            };
            nativeBuildInputs = [protobuf];
            pname = "example-rust";
            src = ./example/rust;
            version = "0.1.0";
          };

          sdk-rust = buildRustPackage {
            cargoLock = {
              lockFile = ./rust/Cargo.lock;
              outputHashes = {
                "vorpal-0.1.0" = "sha256-yps0MqnBtA1xF+0ci8V7GDFxvoxfd3ufKMlHkW9bQSM=";
              };
            };
            nativeBuildInputs = [protobuf];
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
