{ pkgs ? import ./pkgs.nix, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  deepkey = buildDNA {
    inherit shell;

    name = "deepkey";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
