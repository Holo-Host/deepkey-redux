#!/bin/bash

# create an agent named Jack

curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0","id": "0","method": "admin/agent/add", "params":{"id":"jack_id","name":"jack"}}' http://localhost:3000
