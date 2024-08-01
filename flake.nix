{

  inputs = {
    # Convenience functions for writing flakes
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = github:nixos/nixpkgs/nixos-23.11;
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          name = "fprust";
          buildInputs = with pkgs; [
            althttpd
          ];
        };
      }
    );
}
