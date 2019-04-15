const sleep = require('sleep');

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", async(t, { liza }) => {

    const keyset_root_address = liza.call("deepkey", "init", {})
    console.log("My keyset_root_address : ",keyset_root_address.Ok);
    t.ok(keyset_root_address.Ok)

  })
}
