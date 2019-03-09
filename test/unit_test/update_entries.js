const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "set_keyset_root", {})
}

module.exports = (scenario) => {
//   scenario.runTape("testing checks if entries have been pushed", async(t, { liza }) => {
// // On genesis we have to make this call
//     let address = genesis(liza)
//     // sleep.sleep(5);
//
//     let address_recheck = genesis(liza)
//     t.deepEqual(address.Ok, address_recheck.Ok )
//   })
//
//   scenario.runTape("create rules befor the keyset_root should throw an error", async(t, { liza }) => {
//
//   // This is to just test out if we get the right keyset_root address
//     const keyset_root_address = liza.call("deepkey", "get_my_keyset_root", {})
//     console.log("My KeysetRoot Address: ",keyset_root_address);
//     t.deepEqual(keyset_root_address.Err.Internal,  'handle_get_my_keyset_root: No KeysetRoot Exists' )
//
//   })


  scenario.runTape("create", async(t, { liza }) => {

    let address = genesis(liza)

    const check_rules = liza.call("deepkey", "get_rules", {})
    console.log("Error: ",check_rules.Err.Internal);
    t.deepEqual(check_rules.Err.Internal,'handle_get_my_rules: No Rules Exists' )

// Its now time to commit your rules
    const rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Revocation--------------Key"})
    t.deepEqual(rule_commit.Ok,"QmQ2jjN1j48MsLhVVuUiqaFuYCFZr26FzncuwKVd8ja8mg" )


    sleep.sleep(5);
// Check if your getting the right hash
    const my_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok.App[1]);
    t.deepEqual(my_rules.Ok.App[0],"rules" )


    const updated_rule_commit = liza.call("deepkey", "set_rules", {revocation_key:"Updated_Revocation--------------Key"})
    t.deepEqual(updated_rule_commit.Ok,"QmbWeeogmEyq9v8JZtQwnire8uYpwoz7ABoWrJEcPsJSy1" )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_updated_rules = liza.call("deepkey", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok.App[1]);
    t.deepEqual(my_updated_rules.Ok.App[0],"rules" )


// Lets create an authorizor key
    const authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Authorizor------------Key"})
    t.deepEqual(authorizor_commit.Ok,"QmfXxo8DeuELaWEsG9uYXMo2f5g9SmccfGU6cwRymVZXoC" )

// Check if the key exist for the authorizor
    const checking_authorizor_key = liza.call("deepkey", "key_status", {key:"Authorizor------------Key"})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

    sleep.sleep(5);

// Lets create an authorizor key
    const updated_authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Updated_Authorizor------------Key"})
    t.deepEqual(updated_authorizor_commit.Ok,"QmT7g4W6DjmXeJQ3K3VkECYqfAiVfxHCyxwUT7dm1BeJgW" )

    sleep.sleep(5);
// Check if the key exist for the authorizor
    const checking_old_authorizor_key = liza.call("deepkey", "key_status", {key:"Authorizor------------Key"})
    t.deepEqual(checking_old_authorizor_key.Ok,"Doesn\'t Exists" )

  })
}
