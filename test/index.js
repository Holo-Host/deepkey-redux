/** Original Testing without manually setting up conductor */

const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))
const dnaPath = "./dist/DeepKey.dna.json"
const agentLiza = Config.agent("liza")
const dna = Config.dna(dnaPath,'deepkey')
const instanceLiza = Config.instance(agentLiza, dna)
const scenario = new Scenario([instanceLiza], { debugLog: true })

// require('./unit_test/genesis')(scenario);
// require('./unit_test/update_auth_entries')(scenario);
require('./unit_test/test_trait')(scenario);

// require('./unit_test/test_converse')(scenario);

/** Testing with a manual conductor **/

// require('./manual');
