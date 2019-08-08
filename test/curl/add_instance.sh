#!/bin/bash

# Add instance of holofuel


echo "========================================================================"
echo ""
echo "Adding Instance of holofuel..."
echo ""
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0","id": "0","method": "admin/instance/add", "params":{"id":"holofuel-instance","agent_id":"jack_id_","dna_id":"holofuel"}}' http://localhost:9300
echo ""
echo "========================================================================"
echo ""
echo "Getting list of all instances..."
echo ""
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0","id": "0","method": "admin/instance/list", "params":{}}' http://localhost:9300
echo ""
echo "========================================================================"
