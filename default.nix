let
 holonix-release-tag = "0.0.18";
 holonix-release-sha256 = "1njv2zr9f3kcz5v1p3ff6z4gp49k5l9knh2sss6f71sgmsr5q3zm";

 holonix = import (fetchTarball {
  url = "https://github.com/holochain/holonix/tarball/${holonix-release-tag}";
  sha256 = "${holonix-release-sha256}";
 });
 # uncomment to work locally
 # holonix = import ../holonix;
in
with holonix.pkgs;
{
 core-shell = stdenv.mkDerivation (holonix.shell // {
  name = "holochain-serialization-shell";

  buildInputs = []
   ++ holonix.shell.buildInputs

   ++ (holonix.pkgs.callPackage ./install {
    pkgs = holonix.pkgs;
   }).buildInputs

   ++ (holonix.pkgs.callPackage ./test {
    pkgs = holonix.pkgs;
   }).buildInputs
  ;
 });
}
