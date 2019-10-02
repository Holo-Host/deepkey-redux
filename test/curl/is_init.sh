#!/bin/bash


echo "========================================================================"
echo ""
echo "Getting Authorizor..."
echo ""

curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "dpki_happ", "zome": "dpki", "function": "is_initialized", "args": {} }}' http://127.0.0.1:8800


echo ""
echo "========================================================================"
