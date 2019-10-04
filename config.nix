{
 buildInputs = [ ];

 holonix = {

  use-github = true;

  github = {

   ref = "v0.0.36";

   sha256 = "10wslqp5h8fypjp9f4bwqv0qgx9kzwk2092nkn3s3dcivlyjgav4";

   owner = "holochain";

   repo = "holonix";
  };

 };

 release = {
  hook = {
   preflight = ''
hn-release-hook-preflight-manual
'';

   version = ''
hn-release-hook-version-readme
'';

   publish = ''
echo "All finished!!!"
'';
  };

  commit = "_";

  version = {
   current = "0.0.1";
   previous = "0.0.0";
  };

  github = {
   template = "";

   owner = "Holo-Host";

   repo = "DeepKey";

   upstream = "origin";
  };
 };
}
