{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs =
    inputs@{
      parts,
      nci,
      treefmt-nix,
      pre-commit-hooks-nix,
      ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [
        treefmt-nix.flakeModule
        pre-commit-hooks-nix.flakeModule
        nci.flakeModule
        ./crates.nix
      ];
      perSystem =
        {
          pkgs,
          config,
          ...
        }:
        let
          # shorthand for accessing this crate's outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          crateOutputs = config.nci.outputs."stm32exp";
        in
        {
          treefmt = {
            projectRootFile = "flake.nix";
            programs.rustfmt.enable = true;
            programs.nixfmt.enable = true;
            programs.toml-sort.enable = true;
          };
          formatter = config.treefmt.build.wrapper;

          # export the crate devshell as the default devshell
          devShells.default = crateOutputs.devShell.overrideAttrs (old: {
            packages = with pkgs; (old.packages or [ ]) ++ [
              minicom
              probe-rs-tools
              stlink
            ];
          });
          # export the release package of the crate as default package
          packages.default = crateOutputs.packages.release;
        };
    };
}
