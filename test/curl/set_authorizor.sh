#!/bin/bash


echo "========================================================================"
echo ""
echo "Setting Authorizor..."
echo ""

curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "dpki_happ", "zome": "dpki", "function": "set_authorizor", "args": {"authorization_key_path":1,"signed_auth_key":"cjkp7x6lI39sEV5DYBFHtEBqnCjk0khHBtw/tmd6+XD9gsHCDikJTuTJor5NqcxOfapstdT5f3DZVbXjs4xxAQ=="} }}' http://127.0.0.1:8800

echo ""
echo "========================================================================"
