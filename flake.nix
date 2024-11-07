{
  description = "Simple Emoji Picker";

  inputs = {
    nixpkgs.url = "flake:nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
      myrust = (pkgs.rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default.override {
               extensions = [ "rust-analyzer" "rust-src" ];
               }));
    in
      { packages.${system}.default = pkgs.callPackage ./nix/package.nix {};
        devShells.${system}.default = pkgs.callPackage ./nix/devshell.nix {
          rust = myrust;
        };

      };
}
