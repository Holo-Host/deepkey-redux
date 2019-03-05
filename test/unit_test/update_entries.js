const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "set_keyset_root", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing checks if entries have been pushed", (t, { liza }) => {
// On genesis we have to make this call
    let address = genesis(liza)
    // sleep.sleep(5);

    let address_recheck = genesis(liza)
    t.deepEqual(address.Ok, address_recheck.Ok )


  })
}
