const path = require('path')
const { Config } = require('@holochain/try-o-rama')

const dnaPath = path.join(__dirname, "../dist/DeepKey.dna.json")
const dna = Config.dna(dnaPath, 'deepkey')

const config = agentName => ({
  instances: [{
    id: 'deepkey',
    agent: {
      id: agentName,
      name: `${agentName}-${Math.floor(Math.random() * 100000)}`,
      keystore_file: "path/to/keystore",
      public_address: "HcS______________________",
    },
    dna: {
      id: 'deepkey',
      file: dnaPath,
    }
  }],
  dpki: {
    instance_id: 'deepkey',
    init_params: {}
  }
})

module.exports = { config }
