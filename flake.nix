{
  description = "A simple tool to execute a program at the given time";

  inputs = {
    flake-utils = {
      url = github:numtide/flake-utils;
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "wecker";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          src = ./.;
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
