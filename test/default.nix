{ pkgs }:
let
  name = "hf-test";

  script = pkgs.writeShellScriptBin name
  ''
  hf-test-unit
  hf-test-e2e
  '';
in
{
 buildInputs = [
  script
 ]
 ++ (pkgs.callPackage ./unit { }).buildInputs
 ++ (pkgs.callPackage ./e2e { }).buildInputs
 ;
}
