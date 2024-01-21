{ pkgs }:
pkgs.stdenv.mkDerivation {
  __noChroot = true;
  name = "espup";
  buildInputs = with pkgs; [ cacert espup cargo rustc rustup ];
  unpackPhase = "true";
  buildPhase = ''
    export HOME=$out
    export RUSTUP_HOME=$out/bin
    export CARGO_HOME=$out/bin
    espup install -a $out/bin -f $out/bin/esptools.sh
    chmod +x $out/bin/esptools.sh
  '';
  installPhase = ''
    . $out/bin/esptools.sh
  '';
}

