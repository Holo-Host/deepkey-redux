const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/bundle.json"
const agentLiza = Config.agent("liza")
const dna = Config.dna(dnaPath)
const instanceLiza = Config.instance(agentLiza, dna)
const scenario = new Scenario([instanceLiza], { debugLog: true })

// require('./unit_test/genesis')(scenario);
require('./unit_test/update_entries')(scenario);
