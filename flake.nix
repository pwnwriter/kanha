{
  description = "kanha";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ] (system: function nixpkgs.legacyPackages.${system});

      darwinDeps =
        pkgs: with pkgs; [
          darwin.apple_sdk.frameworks.SystemConfiguration
          libiconv
        ];
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          name = "bwatch";
          packages =
            (with pkgs; [
              # cargo
              # cargo-edit
              # clippy
              # rustc
            ])
            ++ (pkgs.lib.optional pkgs.stdenvNoCC.isDarwin (darwinDeps pkgs));
        };
      });
      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
      packages = forAllSystems (pkgs: {
        bwatch =
          with pkgs;
          let
            fs = lib.fileset;
            sourceFiles = fs.unions [
              ./Cargo.lock
              ./Cargo.toml
              ./src
            ];

            cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            cargoLock.lockFile = ./Cargo.lock;
            darwinBuildInputs = (darwinDeps pkgs);
          in
          pkgs.rustPlatform.buildRustPackage {
            inherit pname version cargoLock;
            src = fs.toSource {
              root = ./.;
              fileset = sourceFiles;
            };
            nativeBuildInputs = [
              clippy
              rustfmt
              openssl
            ];
            buildInputs = [ ] ++ lib.optionals stdenv.isDarwin darwinBuildInputs;
            preBuildPhases = [ "cargoFmt" ];
            cargoFmt = ''
              cargo fmt --manifest-path ./Cargo.toml --all --check
            '';
            # right after checkPhase (tests)
            preInstallPhases = [ "clippy" ];
            clippy = ''
              cargo clippy -- --deny warnings
            '';
          };
        default = self.packages.${pkgs.system}.bwatch;
      });
      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.bwatch}/bin/bwatch";
        };
      });
    };
}
