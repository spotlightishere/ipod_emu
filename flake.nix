{
  description = "Toy iPod BootROM emulator";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    all-systems.url = "github:nix-systems/default";
  };

  outputs = inputs@{ self, all-systems, nixpkgs }:
    let
      allSystems = nixpkgs.lib.genAttrs (import inputs.all-systems);
    in
    {
      devShells = allSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in {
          default = with pkgs; mkShell {
            buildInputs = [
              # Necessary to build the unicorn-engine crate
              # against the native library within nixpkgs
              libiconv
              pkg-config
              unicorn

              # General Rust-related development
              cargo
              rustc
              rustfmt
              rustPackages.clippy
              rust-analyzer
            ];

            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        });

      formatter = allSystems (system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt);
    };
}
