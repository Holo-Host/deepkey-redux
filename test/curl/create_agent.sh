#!/bin/bash


echo "========================================================================"
echo ""
echo "Creating an agent via HC..."
echo ""

curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "dpki_happ", "zome": "dpki", "function": "create_agent_key", "args": {"agent_name":"jack_id_"} }}' http://127.0.0.1:8800

echo ""
echo "========================================================================"
