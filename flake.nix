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
          sdk-rust = buildRustPackage {
            cargoSha256 = "sha256-K9yk9ebha0RsNmQdYiOhqdcXs2qtozj+jSu7BPAKQdw=";
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
