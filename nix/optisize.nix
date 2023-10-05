{ rustPlatform
, version ? "git"
, lib
, makeWrapper
, withHandbrake ? true
, handbrake
}:
rustPlatform.buildRustPackage {
  pname = "optisize";
  inherit version;
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ makeWrapper ];
  postInstall = with lib;
    let
      runtimePaths = [ ]
        ++ optional withHandbrake handbrake;
    in
    ''
      wrapProgram $out/bin/optisize \
        --prefix PATH : "${makeBinPath runtimePaths}"
    '';

  meta = {
    description = "CLI utility to optimize file sizes by matching appropriate optimization scripts";
    homepage = "https://github.com/equirosa/optisize";
    license = lib.licenses.agpl3;
    maintainers = with lib.maintainers; [ equirosa ];
  };
}

