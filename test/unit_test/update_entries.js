const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "init", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing checks if entries have been pushed", async(t, { liza }) => {
// On genesis we have to make this call
    let address = genesis(liza)
    // sleep.sleep(5);

    let address_recheck = genesis(liza)
    t.deepEqual(address.Ok, address_recheck.Ok )
  })

  scenario.runTape("create rules befor the keyset_root should throw an error", async(t, { liza }) => {

  // This is to just test out if we get the right keyset_root address
    const keyset_root_address = liza.call("deepkey", "get_initialization_data", {})
    console.log("My KeysetRoot Address: ",keyset_root_address);
    t.deepEqual(keyset_root_address.Err.Internal,  'fn handle_get_my_keyset_root(): No KeysetRoot Exists' )

  })


  scenario.runTape("create", async(t, { liza }) => {

    genesis(liza)

    const check_rules = liza.call("deepkey", "get_rules", {})
    console.log("Error: ",check_rules.Err.Internal);
    t.deepEqual(check_rules.Err.Internal,'handle_get_my_rules: No Rules Exists' )

// Its now time to commit your rules
    const rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Revocation--------------Key"})
    t.ok(rule_commit.Ok)


    sleep.sleep(5);
// Check if your getting the right hash
    const my_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok.App[1]);
    t.deepEqual(my_rules.Ok.App[0],"rules" )


    const updated_rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Updated_Revocation--------------Key"})
    t.ok(updated_rule_commit.Ok )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_updated_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok.App[1]);
    t.deepEqual(my_updated_rules.Ok.App[0],"rules" )


// Lets create an authorizor key
    const authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Authorizor------------Key"})
    t.ok(authorizor_commit.Ok)

// Check if the key exist for the authorizor
    const checking_authorizor_key = liza.call("deepkey", "key_status", {key:"Authorizor------------Key"})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

    sleep.sleep(5);

// Lets create an authorizor key
    const updated_authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Updated_Authorizor------------Key"})
    t.ok(updated_authorizor_commit.Ok)

    sleep.sleep(5);
// Check if the key exist for the authorizor
    const checking_old_authorizor_key = liza.call("deepkey", "key_status", {key:"Authorizor------------Key"})
    t.deepEqual(checking_old_authorizor_key.Ok,"Doesn\'t Exists" )

  })
}
