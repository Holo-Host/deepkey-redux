#!/bin/bash

# create an agent named Jack

echo "========================================================================"
echo ""
echo "Creating agent via admin interface..."
echo ""
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0","id": "0","method": "admin/agent/add", "params":{"id":"jack_id_","name":"AgntJack"}}' http://localhost:9300
echo ""
echo "Agent Created Sucessfully"
echo ""
echo "========================================================================"
echo ""
echo "Getting list of all agent..."
echo ""
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0","id": "0","method": "admin/agent/list", "params":{}}' http://localhost:9300
echo ""
echo "========================================================================"
