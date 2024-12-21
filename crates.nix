{ ... }:
{
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    let
      projectName = "remap";
      g474Name = "stm32g474re";
      f767Name = "stm32f767zi";
    in
    {
      nci.toolchainConfig = ./rust-toolchain.toml;

      # declare projects
      nci.projects.${projectName} = {
        path = ./.;
        export = true;
      };

      # configure crates
      nci.crates = {
        ${g474Name} = { };
        ${f767Name} = { };
      };
    };
}
