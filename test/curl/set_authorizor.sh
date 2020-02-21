#!/bin/bash


echo "========================================================================"
echo ""
echo "Setting Authorizor..."
echo ""

curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "dpki_happ", "zome": "dpki", "function": "set_authorizor", "args": {"authorization_key_path":1,"signed_auth_key":"CzFHUTnO0vYLo8FgOSjessHTRgGePLiIjBYqHSI85M1R/wIMFNdg5k8uKNmpBJi5JqvUvofc5LbTu3XZ/UpwCg=="} }}' http://127.0.0.1:9300

echo ""
echo "========================================================================"
