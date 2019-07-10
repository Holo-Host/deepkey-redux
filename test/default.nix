{ pkgs }:
let
  name = "dk-test";

  script = pkgs.writeShellScriptBin name
  ''
  dk-test-unit
  dk-test-e2e
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
