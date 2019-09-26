const path = require('path')

const { Orchestrator, tapeExecutor } = require('@holochain/try-o-rama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const orchestrator = new Orchestrator({
  middleware: tapeExecutor(require('tape')),
  globalConfig: {
    network: 'n3h',
    logger: false
  }
})

const multi_orchestrator = new Orchestrator({
  middleware: tapeExecutor(require('tape')),
  globalConfig: {
    network: 'n3h',
    logger: false
  }
})

require('./unit_test/update_auth_entries')(orchestrator.registerScenario);
require('./unit_test/set_up_conductor')(orchestrator.registerScenario);
require('./unit_test/revoke_rev_key')(orchestrator.registerScenario);
require('./unit_test/test_init')(orchestrator.registerScenario);
require('./unit_test/notification')(multi_orchestrator.registerScenario);

orchestrator.run()
multi_orchestrator.run()
