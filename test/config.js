const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaName = "DeepKey"
const dnaId = "dpki_happ"

const dnaPath = path.join(__dirname, `../dist/${dnaName}.dna.json`)
const device1Path = path.join(__dirname, "../device-1-n.key")
const device2Path = path.join(__dirname, "../device-2-n.key")

const dna = Config.dna(dnaPath, dnaId)

const networkType = process.env.APP_SPEC_NETWORK_TYPE || "sim2h"
let network = {}
// override the transport_config if we are in the Final Exam context!
if (process.env.HC_TRANSPORT_CONFIG) {
    network=require(process.env.HC_TRANSPORT_CONFIG)
} else {
    network =
        ( networkType === 'websocket'
          ? Config.network('websocket')

          : networkType === 'memory'
          ? Config.network('memory')

          : networkType === 'sim1h'
          ? {
              type: 'sim1h',
              dynamo_url: 'http://localhost:8000'
          }

          : networkType === 'sim2h'
          ? {
              type: 'sim2h',
              sim2h_url: 'wss://localhost:9000'
          }

          : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
        )
}

const logger = {
  type: 'debug',
  rules: {
    rules: [
      {
        exclude: true,
        pattern: '.*parity.*'
      },
      {
        exclude: true,
        pattern: '.*mio.*'
      },
      {
        exclude: true,
        pattern: '.*tokio.*'
      },
      {
        exclude: true,
        pattern: '.*hyper.*'
      },
      {
        exclude: true,
        pattern: '.*rusoto_core.*'
      },
      {
        exclude: true,
        pattern: '.*want.*'
      },
      {
        exclude: true,
        pattern: '.*rpc.*'
      }
    ]
  },
  // state_dump: true
}

const commonConfig = { logger, network }


const simple_conductor_config = (agent) => Config.gen([{
    id: 'dpki_happ',
    agent: {
      id: agent,
      name: `${agent}-${Math.floor(Math.random() * 100000)}`,
      keystore_file: device1Path,
      public_address: "HcSCjJjIe3sRps4zkoCXuu7sUmEdcc6ncH8uID9fMyy7do8ttaciHiZCibgcvrr",
    },
    dna: {
      id: 'deepkey',
      file: dnaPath,
    }
  }],
    commonConfig
    // dpki: {
      //   instance_id: 'dpki_happ',
      //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
      // }
  )

const simple_2_conductor_config = (agent) => Config.gen([{
    id: 'dpki_happ',
    agent: {
      id: agent,
      name: `${agent}-${Math.floor(Math.random() * 100000)}`,
      keystore_file: device2Path,
      public_address: "HcSCJ9rxPzSwzdqhaprQGkXIzJmmc9r9gq4AgGIcvIvjdftfF8HfHw6k8P6Akjr",
    },
    dna: {
      id: 'deepkey',
      file: dnaPath,
    }
  }],
    commonConfig,
    // dpki: {
    //   instance_id: 'dpki_happ',
    //   init_params: {"revocation_key": "HcSCI7fRqt5wb7r6i46f5AeGW6zcNuq3i94fQVtFOPromhzoukr9DabcZqzxzir","signed_auth_key":"bQNCtt9Xa7Ii4mCgOGSt8InVLA6HbrFjhYBoc4lDKMtxbY65kQoMNR/mHCuBq5rBYtyaZXG9Jpa9o8WD2eSrCw=="}
    // }
  )

// Send a newline to the process to enter a null passphrase when prompted
const handleHack = handle => {
  handle.stdin.setEncoding('utf-8')
  handle.stdin.write('\n')
  handle.stdin.end()
}

module.exports = { simple_conductor_config, simple_2_conductor_config, handleHack }
