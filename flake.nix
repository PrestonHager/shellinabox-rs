{
  description = "shellinabox-rs development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.shellinabox-rs = pkgs.rustPlatform.buildRustPackage {
          pname = "shellinabox-rs";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = with pkgs; [ wasm-pack nodejs pkg-config openssl ];
          preBuild = ''
            export RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""
            (cd web && wasm-pack build --release --target web --out-dir ../static/pkg)
          '';
        };

        defaultPackage = self.packages.${system}.shellinabox-rs;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.wasm-pack
            pkgs.nodejs
            pkgs.pkg-config
            pkgs.openssl
          ];
          shellHook = ''
            export RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""
          '';
        };
      });
}
