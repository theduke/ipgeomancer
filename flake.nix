{
  description = "ipgeomancer flake providing ipgeom CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        version = (builtins.fromTOML (builtins.readFile ./ipgeom_cli/Cargo.toml)).package.version;
      in {
        packages.ipgeom = pkgs.rustPlatform.buildRustPackage {
          pname = "ipgeom";
          inherit version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "ipgeom_cli" ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          doCheck = false;
          meta.mainProgram = "ipgeom";
        };
        packages.default = self.packages.${system}.ipgeom;
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.ipgeom;
        };
      });
}
