{
  description = "Ygégé - Indexeur haute performance pour YGG Torrent";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;

          src = pkgs.lib.cleanSource ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            cmake
            perl
            pkg-config
            git
            rustPlatform.bindgenHook 
          ];

          buildInputs = with pkgs; [
            openssl 
          ];

          env = {
            BUILD_COMMIT = self.rev or "dirty";
            BUILD_DATE = "nix-build";
            BUILD_BRANCH = "nix";
          };

          doCheck = false;

          meta = with pkgs.lib; {
            description = manifest.description or "High-performance indexer for YGG Torrent";
            homepage = "https://github.com/UwUDev/ygege";
            license = licenses.mit;
            mainProgram = "ygege";
            platforms = platforms.linux;
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
      }
    );
}