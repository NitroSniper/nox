{
  description = "A flake for developing python application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        # https://wiki.nixos.org/wiki/Python#Possible_Optimizations
        pkgs = import inputs.nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            python3Packages.python-lsp-server
            python3Packages.pylsp-rope
            python3Packages.python-lsp-ruff
          ];
        };
      }
    );
}
