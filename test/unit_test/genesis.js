const sleep = require('sleep');

function genesis (liza){
  return liza.call("deepkey", "init", {revocation_key: "Revocation...............Key"})
}

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", async(t, { liza }) => {
// On genesis we have to make this call
    let address = genesis(liza)
    sleep.sleep(5);
// This is to just test out if we get the right keyset_root address
    const keyset_root_address = liza.call("deepkey", "get_initialization_data", {})
    t.equal(keyset_root_address.Ok,address.Ok)

// Lets create an authorizor key
    const authorizor_commit = liza.call("deepkey", "set_authorizor", {authorization_key:"Authorizor------------Key"})
    console.log("------->",authorizor_commit);
    t.ok(authorizor_commit.Ok)

// Check if the key exist for the authorizor
    const checking_authorizor_key = liza.call("deepkey", "key_status", {key:"Authorizor------------Key"})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

// Check if the key exist for the key
// This is befor this is created
    const checking_key_1 = liza.call("deepkey", "key_status", {key:"Agent------------Key"})
    t.deepEqual(checking_key_1.Ok,"Doesn't Exists" )

// Lets create an agent key
    const key_commit = liza.call("deepkey", "set_key", {new_agent_key:"Agent------------Key"})
    t.ok(key_commit.Ok)

// Check if the key exist for the key
// Now it should exist
    const checking_key_2 = liza.call("deepkey", "key_status", {key:"Agent------------Key"})
    t.deepEqual(checking_key_2.Ok,"live" )

  })
}
