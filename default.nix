{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  emacs-with-htmlize = emacsWithPackages (epkgs: with epkgs; [
    htmlize
  ]);
in

{
  DeepKey = buildDNA {
    name = "DeepKey";
    src = gitignoreSource ./.;
  };

  DeepKey-docs = stdenv.mkDerivation {
    name = "DeepKey-docs";
    src = gitignoreSource ./.;

    nativeBuildInputs = [ emacs-with-htmlize ];
    makeFlags = [ "doc-all" ];

    installPhase = ''
      mkdir -p $out/nix-support
      echo "doc manual $out" > $out/nix-support/hydra-build-products
      mv doc/*.html $out
    '';
  };
}
