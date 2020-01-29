const { Orchestrator, tapeExecutor, combine, callSync, localOnly } = require('@holochain/tryorama')

const MIN_EXPECTED_SCENARIOS = 1

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error)
})

const middleware = combine(
  // by default, combine conductors into a single conductor for in-memory networking
  // NB: this middleware makes a really huge difference! and it's not very well tested,
  // as of Oct 1 2019. So, keep an eye out.
  tapeExecutor(require('tape')),
  localOnly,
  callSync
)

const orchestrator = new Orchestrator({
  middleware,
  waiter: {
    softTimeout: 5000,
    hardTimeout: 10000
  }
})

// These tests are using manual setup
// (i.e. they do not use the dpki setting in the holochain conductor)
require('./unit_test/manual_dpki/update_auth_entries')(orchestrator.registerScenario)
require('./unit_test/manual_dpki/set_up_conductor')(orchestrator.registerScenario)
require('./unit_test/manual_dpki/revoke_rev_key')(orchestrator.registerScenario)
require('./unit_test/manual_dpki/test_init')(orchestrator.registerScenario)
// require('./unit_test/manual_dpki/notification')(orchestrator.registerScenario)

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
// require('./unit_test/auto_dpki/test_init')(set_orchestrator.registerScenario)
// Check to see that we haven't accidentally disabled a bunch of scenarios

const num = orchestrator.numRegistered()
if (num < MIN_EXPECTED_SCENARIOS) {
  console.error(`Expected at least ${MIN_EXPECTED_SCENARIOS} scenarios, but only ${num} were registered!`)
  process.exit(1)
}
else {
  console.log(`Registered ${num} scenarios (at least ${MIN_EXPECTED_SCENARIOS} were expected)`)
}

orchestrator.run().then(stats => {
  console.log("All done.")
})
