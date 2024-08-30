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
        utilities = with pkgs; [
          python3Packages.python-lsp-server
          python3Packages.pylsp-rope
          python3Packages.python-lsp-ruff
        ];
      in
      {
        devShells =
          let
            util = pkgs.mkShell { packages = utilities; };
            battery = pkgs.mkShell { packages = utilities; };
            chain = pkgs.mkShell { };
          in
          {
            inherit battery chain util;
            default = chain;
          };
      }
    );
}
