const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/DeepKey.dna.json")
const dna = Diorama.dna(dnaPath, 'deepkey')

const singleInstance = new Diorama({
  instances: {
    liza: dna,
  },
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

const multiInstance = new Diorama({
  instances: {
    liza: dna,
    jack: dna,
  },
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

require('./unit_test/update_auth_entries')(singleInstance.registerScenario);
require('./unit_test/set_up_conductor')(singleInstance.registerScenario);
require('./unit_test/revoke_rev_key')(singleInstance.registerScenario);
require('./unit_test/notification')(multiInstance.registerScenario);
require('./unit_test/test_init')(multiInstance.registerScenario);

singleInstance.run()
multiInstance.run()
