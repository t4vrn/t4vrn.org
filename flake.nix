{
  description = "t4vrn.org";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with WASM support
            rustToolchain

            # Node.js for Tailwind CSS
            nodejs_20

            # Build tools
            pkg-config
            openssl

            # Development tools
            git

            dioxus-cli
            binaryen # wasm-opt
          ];

          shellHook = ''
            # Check if node_modules exists
            if [ ! -d "node_modules" ]; then
              echo "⚠️  Run 'npm install' to install dependencies"
            fi
          '';
        };
      }
    );
}
