{ lib, pkgs, }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "embuild";
  version = "0.31.4"; # Replace with the appropriate version

  src = pkgs.fetchFromGitHub {
    owner = "esp-rs"; # Replace with the appropriate repository owner
    repo = pname;
    rev = "v${version}";
    sha256 =
      "sha256-YH2CPb3uBlPncd+KkP25xhCVvDB7HDxJuSqWOJ1LT3k="; # Put the correct sha256 hash here
  };

  cargoSha256 =
    "sha256-nlCnLBdBVNyRKM3gjPXP7I1PJjBLKR6fFbKstkqAQeE="; # Put the correct cargoSha256 hash here

  # Specify any Rust build dependencies here
  buildInputs = [ ];

  # Set custom build flags if necessary
  cargoBuildFlags = [ "-p" "cargo-pio" "-p" "ldproxy" ];

  meta = with lib; {
    description = "A description of the embuild package";
    homepage =
      "https://github.com/embuild/embuild"; # Replace with the correct URL
    license = licenses.mit; # Replace with the correct license
    maintainers = [ "MunsMan" ]; # Add maintainers here
  };
}

