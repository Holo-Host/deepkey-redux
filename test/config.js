const path = require('path')
const { Config } = require('@holochain/try-o-rama')

const dnaPath = path.join(__dirname, "../dist/DeepKey.dna.json")
const dna = Config.dna(dnaPath, 'deepkey')

const config = {
  instances: {
    deepkey: dna
  },
  dpki: {
    instance_id: 'deepkey',
    init_params: {}
  }
}

module.exports = { config }
