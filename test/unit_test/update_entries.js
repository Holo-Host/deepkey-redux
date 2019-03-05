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

  scenario.runTape("create rules befor the keyset_root should throw an error", (t, { liza }) => {

  // This is to just test out if we get the right keyset_root address
    const keyset_root_address = liza.call("deepkey", "get_my_keyset_root", {})
    console.log("My KeysetRoot Address: ",keyset_root_address);
    t.deepEqual(keyset_root_address.Err.Internal,  'handle_get_my_keyset_root: No KeysetRoot Exists' )

  })


  scenario.runTape("create", (t, { liza }) => {

    let address = genesis(liza)

    const check_rules = liza.call("deepkey", "get_rules", {})
    console.log("Error: ",check_rules.Err.Internal);
    t.deepEqual(check_rules.Err.Internal,'handle_get_my_rules: No Rules Exists' )

// Its now time to commit your rules
    const rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Revocation--------------Key"})
    t.deepEqual(rule_commit.Ok,"QmauGn7nkmpnDcY1wz5W7Fi2pT1tupeGz16Ss8m1AKQUu4" )


    sleep.sleep(5);
// Check if your getting the right hash
    const my_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok.App[1]);
    t.deepEqual(my_rules.Ok.App[0],"rules" )


    const updated_rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Updated_Revocation--------------Key"})
    t.deepEqual(updated_rule_commit.Ok,"QmVnEqx7tFv1a6nW13cUoWeyDWb1Vus9kdcosQdWVEU6Wj" )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_updated_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok.App[1]);
    t.deepEqual(my_updated_rules.Ok.App[0],"rules" )

  })
}
