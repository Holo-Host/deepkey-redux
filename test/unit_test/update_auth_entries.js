const sleep = require('sleep');
const REVOCATION_KEY = "HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa";
const SIGNED_AUTH_KEY_1 ="LVeIAP0horN0UhEVuqZyDCPjcYzvQUj9AMRm4Hv+xtsS6QoHYUeudekZoVYcPtktf+tDTtP/yFu8O3+jsZDbBQ==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";

async function conductor_init (liza){
  return await liza.call("dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa\"}"})
}

module.exports = (scenario) => {
  scenario("testing checks if entries have been pushed", async(s, t, { liza }) => {
    // On conductor_init we have to make this call
    let address = await conductor_init(liza)
    let address_recheck = await conductor_init(liza)
    t.deepEqual(address.Ok, address_recheck.Ok )
  })

  scenario("testing if create rules before the keyset_root should throw an error", async(s, t, { liza }) => {
  // This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.call("dpki", "get_initialization_data", {})
    console.log("My KeysetRoot Address: ",keyset_root_address);
    t.deepEqual(keyset_root_address.Err.Internal,  'fn handle_get_my_keyset_root(): No KeysetRoot Exists' )
  })


  scenario("testing the initial set up process and trying to update it", async(s, t, { liza }) => {

    await conductor_init(liza)

    sleep.sleep(5)

    const check_rules = await liza.call("dpki", "get_rules", {})
    console.log("Initial Rules: ",check_rules);
    t.deepEqual(check_rules.Ok.length,1 )

// Check if your getting the right hash
    const my_rules = await liza.call("dpki", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0]);
    t.ok(my_rules.Ok[0].entry.revocationKey,REVOCATION_KEY)

// Lets create an authorizor key
    const authorizor_commit = await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:SIGNED_AUTH_KEY_1
    })
    t.ok(authorizor_commit.Ok)

// Check if the key exist for the authorizor
    const not_registered_key = await liza.call("dpki", "key_status", {key:"Not-Registered-Key"})
    t.deepEqual(not_registered_key.Ok,"Doesn\'t Exists" )

// Check if the key exist for the authorizor
    const checking_authorizor_key = await liza.call("dpki", "key_status", {key:authorizor_commit.Ok})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

// Lets create an authorizor key
    const updated_authorizor_commit = await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 2,
      signed_auth_key:SIGNED_AUTH_KEY_2
    })
    t.ok(updated_authorizor_commit.Ok)

    const checking_new_authorizor_key = await liza.call("dpki", "key_status", {key:updated_authorizor_commit.Ok})
    t.deepEqual(checking_new_authorizor_key.Ok,"live" )

    sleep.sleep(5);
// Check if the key exist for the authorizor
    const checking_old_authorizor_key = await liza.call("dpki", "key_status", {key:authorizor_commit.Ok})
    t.deepEqual(checking_old_authorizor_key.Ok,"modified" )

  })
}
