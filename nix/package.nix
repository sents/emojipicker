{ lib
, rustPlatform
, pkg-config
, gtk4
, wrapGAppsHook4
, glib
, clippy
}: rustPlatform.buildRustPackage rec {
  pname = "emojipicker";
  version  = "0.1";

  src = lib.cleanSource ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  doCheck = true;
  nativeBuildInputs = [
    clippy
    pkg-config
    glib
    wrapGAppsHook4
  ];

  buildInputs = [ gtk4 ];
  postCheck = ''
    cargo-clippy
  '';

  meta = with lib; {
    description = "Simple emoji picker using gtk4 and relm";
    homepage = "https://github.com/sents/emojipicker";
    license = licenses.gpl3;
    mainProgram = "emojipicker";
    maintainers = [
      {
        email = "finn@krein.moe";
        github = "sents";
        githubId = 26575793;
        name = "Finn Krein-Schuch";
      }
    ];
  };
}
