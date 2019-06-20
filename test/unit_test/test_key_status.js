const sleep = require('sleep');
const REVOCATION_KEY = "HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa";
const SIGNED_AUTH_KEY_1 ="LVeIAP0horN0UhEVuqZyDCPjcYzvQUj9AMRm4Hv+xtsS6QoHYUeudekZoVYcPtktf+tDTtP/yFu8O3+jsZDbBQ==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";

const APP_KEY_1 = "HcSCJw6d7h53IAh8twROoUTe8qEiibgfxd3AuB9TwU7UktskXWiSyJ6b8334Umz";
const APP_KEY_2 = "HcScIidm755H3Oohd6PFA5TY9ebhxofqpbtZVceQ3yp4p6bbDfaGijB3sbapmii";
const SIGNED_APP_KEY_1_BY_REV_KEY ="b9VltsBRq71nPcJO/EzBz4EtUkqVPNhbS9ggYi90/hldNgHMOETtW19TdLxUXg3VpznjDP6pesyoBpcvzJXsBA==";

async function genesis (liza){
  return await liza.call("dpki", "init", {revocation_key: REVOCATION_KEY})
}

module.exports = (scenario) => {
  scenario("testing out how genesis/init calls should be set up", async(s, t, { liza }) => {
// On genesis we have to make this call
    let address = await genesis(liza)
    sleep.sleep(5);
// This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.call("dpki", "get_initialization_data", {})
    t.equal(keyset_root_address.Ok,address.Ok)

// Lets create an authorizor key
    const authorizor_commit =await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:SIGNED_AUTH_KEY_1
    })
    console.log(authorizor_commit);
    t.ok(authorizor_commit.Ok)

// Check if the key exist for the authorizor
    const checking_authorizor_key = await liza.call("dpki", "key_status", {key:authorizor_commit.Ok})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

// Check if the key exist for the key
// This is befor this is created
    const checking_key_1 = await liza.call("dpki", "key_status", {key:APP_KEY_1})
    t.deepEqual(checking_key_1.Ok,"Doesn't Exists" )

// Lets create an agent key
    const key_commit = await liza.call("dpki", "create_agent_key", {
      derivation_index:1,
      key_type:"AppSig",
      context:"dna12345"
    })
    t.ok(key_commit.Ok)

// Check if the key exist for the key
// Now it should exist
    const checking_key_2 = await liza.call("dpki", "key_status", {key:APP_KEY_1})
    t.deepEqual(checking_key_2.Ok,"live" )

    sleep.sleep(5)

    const updated_key = await liza.call("dpki", "update_key", {
      old_key:APP_KEY_1,
      signed_old_key:SIGNED_APP_KEY_1_BY_REV_KEY,
      new_key:APP_KEY_2,
      derivation_index:2,
      key_type:"AppSig",
      context:"dna12345"
    })
    console.log("Updated Key: ",updated_key);
    t.ok(updated_key.Ok)

    sleep.sleep(5)

// Check if the key exist for the key
// Now the old key should be shown as updated and the new should be live
    const checking_key_3 = await liza.call("dpki", "key_status", {key:APP_KEY_1})
    t.deepEqual(checking_key_3.Ok,"modified" )

    const checking_key_4 = await liza.call("dpki", "key_status", {key:APP_KEY_2})
    t.deepEqual(checking_key_4.Ok,"live" )

  })
}
