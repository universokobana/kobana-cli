{
  description = "Kobana CLI — dynamic command surface from OpenAPI specs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Extract version from CLI crate's Cargo.toml
        cargoToml = builtins.fromTOML (builtins.readFile ./crates/kobana-cli/Cargo.toml);
        version = cargoToml.package.version;

        # System dependencies
        # On Linux, keyring needs libsecret
        # On macOS, it uses Security framework
        linuxDeps = with pkgs; [
          libsecret
        ];

        darwinDeps = with pkgs; [
          libiconv
          apple-sdk
        ];

        kobana = pkgs.rustPlatform.buildRustPackage {
          pname = "kobana";
          inherit version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux linuxDeps
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin darwinDeps;

          doCheck = false;

          meta = with pkgs.lib; {
            description = "Kobana CLI — interact with the Kobana financial API";
            license = licenses.mit;
            mainProgram = "kobana";
          };
        };
      in
      {
        packages.default = kobana;
        packages.kobana = kobana;

        apps.default = flake-utils.lib.mkApp {
          drv = kobana;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ kobana ];
          buildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      }
    );
}
