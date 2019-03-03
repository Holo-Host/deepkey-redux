const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "set_keyset_root", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", (t, { liza }) => {
// On genesis we have to make this call
    let address = genesis(liza)
    sleep.sleep(5);
// This is to just test out if we get the right keyset_root address
    const keyset_root_address = liza.call("deepkey", "get_my_keyset_root", {})
    console.log("My KeysetRoot Address: ",address.Ok);
    t.deepEqual(keyset_root_address.Ok, address.Ok )

// Its now time to commit your rules
    const rule_commit = liza.call("deepkey", "create_rules", {revocation_key:"REVOCATIONKEY"})
    t.deepEqual(rule_commit.Ok,"QmPs1tqhYUTJWnLTMP4Z7kgYD2JrWXHxscVkNaidownpr6" )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0].App);
    t.deepEqual(my_rules.Ok[0].App[0],"rules" )
  })
}
