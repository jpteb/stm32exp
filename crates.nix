{ ... }:
{
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    let
      crateName = "stm32exp";
    in
    {
      nci.toolchainConfig = ./rust-toolchain.toml;
      # declare projects
      nci.projects."stm32exp".path = ./.;
      # configure crates
      nci.crates.${crateName} = { };
    };
}
