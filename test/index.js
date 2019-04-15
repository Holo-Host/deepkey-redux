const { Config, Scenario } = require("../../holochain-rust/nodejs_conductor")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/DeepKey.dna.json"
const agentLiza = Config.agent("liza")
const dna = Config.dna(dnaPath,'deepkey')
const instanceLiza = Config.instance(agentLiza, dna)
const scenario = new Scenario([instanceLiza], { debugLog: false })

// require('./unit_test/genesis')(scenario);
require('./unit_test/update_entries')(scenario);
// require('./unit_test/test_trait')(scenario);


// require('./unit_test/test_converse')(scenario);
