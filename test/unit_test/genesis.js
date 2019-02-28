const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "set_keyset_root", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", (t, { liza }) => {
    const addr = genesis(liza)
    sleep.sleep(5);
    const result = liza.call("deepkey", "get_keyset_root", {"address": addr.Ok})
    t.deepEqual(result.Ok[0].entry.firstDeepkeyAgent,liza.agentId )
  })
}
