const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "set_keyset_root", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", (t, { liza }) => {
    let address = genesis(liza)
    sleep.sleep(5);
    const result = liza.call("deepkey", "get_my_keyset_root", {})
    console.log("KeysetRoot: ",address.Ok);
    t.deepEqual(result.Ok, address.Ok )

    const rule_commit = liza.call("deepkey", "create_rules", {revocation_key:"REVOCATIONKEY"})
    t.deepEqual(rule_commit.Ok,"QmPs1tqhYUTJWnLTMP4Z7kgYD2JrWXHxscVkNaidownpr6" )
    //
    sleep.sleep(5);
    const returned_rule = liza.call("deepkey", "get_rules", {})
    console.log("------------>",returned_rule.Ok[0].App);
    t.deepEqual(returned_rule.Ok[0].App[0],"rules" )

  })
}
