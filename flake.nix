{
  description = "A simple tool to execute a program at the given time";

  inputs = {
    flake-utils.url = github:numtide/flake-utils;
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
          ];
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
