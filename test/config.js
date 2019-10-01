const path = require('path')
const { Config } = require('@holochain/try-o-rama')
const _ = require('lodash')
const dnaPath = path.join(__dirname, "../dist/DeepKey.dna.json")
const device1Path = path.join(__dirname, "../device-1.key")
const device2Path = path.join(__dirname, "../device-2-n.key")
const dna = Config.dna(dnaPath, 'dpki_happ')

const simple_conductor_config = (agent) => ({
  instances: [{
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
  // dpki: {
  //   instance_id: 'dpki_happ',
  //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
  // }
})

const simple_2_conductor_config = (agent) => ({
  instances: [{
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
  // dpki: {
  //   instance_id: 'dpki_happ',
  //   init_params: {"revocation_key": "HcSCI7fRqt5wb7r6i46f5AeGW6zcNuq3i94fQVtFOPromhzoukr9DabcZqzxzir","signed_auth_key":"bQNCtt9Xa7Ii4mCgOGSt8InVLA6HbrFjhYBoc4lDKMtxbY65kQoMNR/mHCuBq5rBYtyaZXG9Jpa9o8WD2eSrCw=="}
  // }
})

// Send a newline to the process to enter a null passphrase when prompted
const handleHack = handle => {
  handle.stdin.setEncoding('utf-8')
  handle.stdin.write('\n')
  handle.stdin.end()
}

const callSyncMiddleware = (run, f) => run(s => {
  const s_ = Object.assign({}, s, {
    players: async (...a) => {
      const players = await s.players(...a)
      const players_ = _.mapValues(
        players,
        api => Object.assign(api, {
          callSync: async (...b) => {
            const result = await api.call(...b)
            await s.consistency()
            return result
          }
        })
      )
      return players_
    }
  })
  return f(s_)
})

module.exports = { simple_conductor_config, simple_2_conductor_config, handleHack, callSyncMiddleware }
