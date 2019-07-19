const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/DeepKey.dna.json")
const dna = Diorama.dna(dnaPath, 'deepkey')

const diorama = new Diorama({
  instances: {
    liza: dna,
    // jack: dna,
  },
  // bridges: [
  //   Diorama.bridge('test-bridge', 'liza', 'jack')
  // ],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

require('./unit_test/update_auth_entries')(diorama.registerScenario);
require('./unit_test/set_up_conductor')(diorama.registerScenario);

diorama.run()

//===========================================
// Old testing fremework
//===========================================
/** Original Testing without manually setting up conductor */
// const { Config, Scenario } = require("../../holochain-rust/nodejs_conductor")
// const { Config, Scenario } = require("@holochain/holochain-nodejs")
// Scenario.setTape(require("tape"))
// const dnaPath = "./dist/DeepKey.dna.json"
// const agentLiza = Config.agent("liza")
// const dna = Config.dna(dnaPath,'dpki')
// const instanceLiza = Config.instance(agentLiza, dna)
// const scenario = new Scenario([instanceLiza], { debugLog: true })

// require('./unit_test/update_auth_entries')(scenario);
// require('./unit_test/test_key_status')(scenario);
// require('./unit_test/test_trait')(scenario);

// require('./unit_test/test_converse')(scenario);

/** Testing with a manual conductor **/

// require('./manual');
