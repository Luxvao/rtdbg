{
  description = "A flake for mmux";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
    in
    {
      devShells.default = with pkgs; mkShell {
        buildInputs = [
          pkg-config
          eza
          rust-bin.stable.latest.default
          rust-analyzer
          wayland
          libxkbcommon
          just
        ];

        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:/run/opengl-driver/lib:$LD_LIBRARY_PATH

          zsh
          exit
        '';
      };
    }
  );
}
