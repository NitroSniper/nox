{
  # https://wiki.nixos.org/wiki/Python#With_pyproject.toml
  # This code uses pyproject.toml, if you want to use something else find it here
  # https://nix-community.github.io/pyproject.nix/introduction.html
  description = "A flake for developing python application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pyproject-nix = {
      url = "github:nix-community/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        # https://wiki.nixos.org/wiki/Python#Possible_Optimizations
        pkgs = import inputs.nixpkgs { inherit system; };

        python = pkgs.python3;

        utilities = with pkgs; [
          python3Packages.python-lsp-server
          python3Packages.pylsp-rope
          python3Packages.python-lsp-ruff
          poetry
        ];

        project = inputs.pyproject-nix.lib.project.loadPyproject {
          # Read & unmarshal pyproject.toml relative to this project root.
          # projectRoot is also used to set `src` for renderers such as buildPythonPackage.
          projectRoot = ./.;
        };

        # Returns an attribute set that can be passed to `buildPythonPackage`.
        attrs = project.renderers.buildPythonPackage { inherit python; };
        bin = python.pkgs.buildPythonPackage attrs;

        # Returns a wrapped environment (virtualenv like) with all our packages
        arg = project.renderers.withPackages { inherit python; };
        venv = python.withPackages arg;

        docker = pkgs.dockerTools.buildImage {
          name = "nox";
          # tag = "latest";
          # TODO! replace foo with package name
          config.Cmd = [ "${bin}/bin/foo" ];
        };
      in
      {
        packages = {
          inherit docker;
          default = bin;
        };

        devShells =
          let
            util = pkgs.mkShell { packages = utilities; };
            battery = pkgs.mkShell { packages = utilities ++ [ venv ]; };
            chain = pkgs.mkShell { packages = [ venv ]; };
          in
          {
            inherit battery chain util;
            default = battery;
          };
      }
    );
}
