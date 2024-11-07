{ lib
, mkShell
, pkg-config
, gtk4
, rust
, glib
}: mkShell rec {
          nativeBuildInputs = [ pkg-config rust glib];
          buildInputs = [
            gtk4
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        }
