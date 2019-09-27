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

// These tests are using manual setup
// (i.e. they do not use the dpki setting in the holochain conductor)
require('./unit_test/manual_dpki/update_auth_entries')(orchestrator.registerScenario);
require('./unit_test/manual_dpki/set_up_conductor')(orchestrator.registerScenario);
require('./unit_test/manual_dpki/revoke_rev_key')(orchestrator.registerScenario);
require('./unit_test/manual_dpki/test_init')(orchestrator.registerScenario);
// require('./unit_test/manual_dpki/notification')(orchestrator.registerScenario);
orchestrator.run()

// These tests have deepkey set as a dpki_instance in the conductor via the dpki settings
// Eg:
// [dpki]
// instance_id = "dpki_happ"
// init_params = "{}"
//
// const set_orchestrator = new Orchestrator({
//   middleware: tapeExecutor(require('tape')),
//   globalConfig: {
//     network: 'n3h',
//     logger: false
//   }
// })
//
// require('./unit_test/auto_dpki/test_init')(set_orchestrator.registerScenario);
