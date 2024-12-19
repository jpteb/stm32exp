{ ... }:
{
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    let
      crateName = "remap";
    in
    {
      nci.toolchainConfig = ./rust-toolchain.toml;
      # declare projects
      nci.projects."remap".path = ./.;
      # configure crates
      nci.crates.${crateName} = { };
    };
}
