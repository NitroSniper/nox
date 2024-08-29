{
  description = "A flake for developing and building rust application to binary";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    # rust dev toolchain
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # cache build steps
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        # Tell Crane to use our toolchain
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.nightly.latest.default.override { }
        );

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the binary itself, reusing the dependency
        # artifacts from above.
        bin = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in
      {
        packages = {
          default = bin;
        };
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            rust-analyzer
            bacon
          ];
        };
      }
    )
    // {
      templates = {
        default = null;

        rust = {
          path = ./rust;
          description = "Rust Environment";
        };
      };
    };
}
