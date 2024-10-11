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
      {
        devShells.${system}.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
            gtk4
            myrust
            gsettings-desktop-schemas
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };

      };
}
