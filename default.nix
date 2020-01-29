{ pkgs ? import ./pkgs.nix {}, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  deepkey = buildDNA {
    inherit shell;

    name = "deepkey";
    src = gitignoreSource ./.;

    nativeBuildInputs = [
      cmake # required by wabt
      binaryen
      wasm-gc
      wabt
    ]
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
