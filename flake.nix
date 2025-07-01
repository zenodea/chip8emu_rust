{
  description = "CHIP-8 Emulator in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustfmt
            clippy
            pkg-config
          ];

          shellHook = ''
            echo "CHIP-8 Emulator Development Environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo run      - Run the emulator"
            echo "  cargo test     - Run tests"
            echo "  cargo clippy   - Run linter"
            echo "  cargo fmt      - Format code"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "chip8emu";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = with pkgs; [
            pkg-config
          ];

          meta = with pkgs.lib; {
            description = "A CHIP-8 emulator written in Rust";
            homepage = "https://github.com/zenodeangeli/chip8emu";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      });
}