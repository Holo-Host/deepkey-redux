const path = require('path')
const {
  Config
} = require('@holochain/tryorama')

const dnaName = "DeepKey"
const dnaId = "dpki_happ"

const dnaPath = path.join(__dirname, `../dist/${dnaName}.dna.json`)
const device1Path = path.join(__dirname, "../test/test-keys/test-agent-1.key")
const device2Path = path.join(__dirname, "../test/test-keys/test-agent-2.key")

const dna = Config.dna(dnaPath, dnaId)

const networkType = process.env.APP_SPEC_NETWORK_TYPE || "sim2h"
let network = {}
// override the transport_config if we are in the Final Exam context!
if (process.env.HC_TRANSPORT_CONFIG) {
  network = require(process.env.HC_TRANSPORT_CONFIG)
} else {
  network =
    (networkType === 'websocket' ?
      Config.network('websocket')

      :
      networkType === 'memory' ?
      Config.network('memory')

      :
      networkType === 'sim1h' ? {
        type: 'sim1h',
        dynamo_url: 'http://localhost:8000'
      }

      :
      networkType === 'sim2h' ? {
        type: 'sim2h',
        sim2h_url: 'ws://localhost:9000'
      }

      :
      (() => {
        throw new Error(`Unsupported network type: ${networkType}`)
      })()
    )
}

const logger = {
  type: 'debug',
  rules: {
    rules: [{
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
      },
      {
        exclude: true,
        pattern: '.*ws.*'
      },
      {
        exclude: true,
        pattern: '.*holochain_net.*'
      },
      {
        exclude: true,
        pattern: '.*holochain_metrics.*'
      }
    ]
  },
  // state_dump: true
}

const commonConfig = {
  logger,
  network,
  passphrase_service: {
    type: 'mock',
    passphrase: ""
  }
}


const simple_conductor_config = (agent) => Config.gen(({
    uuid
  }) => [{
    id: 'dpki_happ',
    agent: {
      id: `${agent}`,
      name: `${agent}-${uuid}`,
      keystore_file: device1Path,
      public_address: "HcScIkJGqVKcw83yv7gfTXJ6c5pUzj9jj6g675gEvKqsUxsxt6cDVX8mGR8d49r",
    },
    dna: {
      id: 'deepkey',
      file: dnaPath,
      uuid,
    }
  }],
  commonConfig
  // dpki: {
  //   instance_id: 'dpki_happ',
  //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
  // }
)

const simple_2_conductor_config = (agent) => Config.gen(({
    uuid
  }) => [{
    id: 'dpki_happ',
    agent: {
      id: `${agent}`,
      name: `${agent}-${uuid}`,
      keystore_file: device2Path,
      public_address: "HcScjD6FeXo7a83p7ubW5uu4iAcF6ij9ymMy8qS6ifsmeo6n7wDna4eeFZv4wgi",
    },
    dna: {
      id: 'deepkey',
      file: dnaPath,
      uuid,
    }
  }],
  commonConfig,
  // dpki: {
  //   instance_id: 'dpki_happ',
  //   init_params: {"revocation_key": "HcSCI7fRqt5wb7r6i46f5AeGW6zcNuq3i94fQVtFOPromhzoukr9DabcZqzxzir","signed_auth_key":"bQNCtt9Xa7Ii4mCgOGSt8InVLA6HbrFjhYBoc4lDKMtxbY65kQoMNR/mHCuBq5rBYtyaZXG9Jpa9o8WD2eSrCw=="}
  // }
)

module.exports = {
  simple_conductor_config,
  simple_2_conductor_config
}