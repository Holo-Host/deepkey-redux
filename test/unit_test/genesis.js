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
    const rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Revocation--------------Key"})
    t.deepEqual(rule_commit.Ok,"QmauGn7nkmpnDcY1wz5W7Fi2pT1tupeGz16Ss8m1AKQUu4" )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0].App);
    t.deepEqual(my_rules.Ok[0].App[0],"rules" )

    const authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Authorizor------------Key"})
    t.deepEqual(authorizor_commit.Ok,"QmQPQfosMrbK8yBAzahaMhsEyC58z4PELZWnfRuE2zsVDo" )

  })
}
