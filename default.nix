let

 config = import ./config.nix;

 holonix = import (
  if ! config.holonix.use-github
  then config.holonix.local.path
  else fetchTarball {
   url = "https://github.com/${config.holonix.github.owner}/${config.holonix.github.repo}/tarball/${config.holonix.github.ref}";
   sha256 = config.holonix.github.sha256;
  }
 ) { config = config; };

in
with holonix.pkgs;
{
 dev-shell = stdenv.mkDerivation (holonix.shell // {
  name = "dev-shell";

  shellHook = holonix.pkgs.lib.concatStrings [''
  # environment variables used by rust tests directly
  export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
  export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
  # config file used by aws cli tool
  export AWS_CONFIG_FILE=`pwd`/.aws/config
  RUST_LOG=sim1h=trace
  ''
  holonix.shell.shellHook
  ];

  buildInputs = [ ]
   ++ holonix.shell.buildInputs
   ++ config.buildInputs
   ++ (holonix.pkgs.callPackage ./.aws {
     pkgs = holonix.pkgs;
    }).buildInputs
  ++ (holonix.pkgs.callPackage ./dynamodb {
     pkgs = holonix.pkgs;
    }).buildInputs

  ;
 });
}
