const { simple_conductor_config, handleHack } = require('../../config')
const sleep  = require('sleep')
const REVOCATION_KEY = "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi";
const SIGNED_AUTH_KEY_1 ="zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";
const AGENT_SIG_KEY_1 = "HcScJuDS4YQYyvoqzbY8WfzfcBr5svueuX3MO7U4D64AMO55qv7ZQYNknrwqjhz";
const SIGNED_AGENT_SIG_KEY_1_BY_REV_KEY ="EgrmcGMkcO7+jW3hUvee4WmNs+k/6QMRFnnZnCKxVnRnB1Dp+mRDUEU2sDQdGHrh5q2vLm4Vs7XgxIAZMcE7AQ==";
const AGENT_ENC_KEY_1 = "HcKciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SIGNED_AGENT_ENC_KEY_1_BY_REV_KEY = "L0ecyb+IE570ckU1Ln6ODLqv5O2ldXafGX26QDbPQeiIsk5dRDbgqYkcSc+oHMJeKORcDjzgDpdCagpXokMbCA==";
const AGENT_SIG_KEY_2 = "HcSCJ6Q45PCMvwdg5rWKjzTVPi9hoo8ixIzSUFh84tNz9hrs8GfP839IiYNT6wi";

async function conductor_init (liza){
  return await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
}

module.exports = (scenario) => {
  scenario("testing out how conductor should be set up", async(s, t) => {

    const { liza } = await s.players({ liza: simple_conductor_config('liza')})

    await liza.spawn(handleHack)

// On conductor_init we have to make this callSync
    let address = await conductor_init(liza)
    t.ok(address)


// This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.callSync('dpki_happ', "dpki", "get_initialization_data", {})
    // add this test when the init is fixed
    // t.equal(keyset_root_address.Ok,address.Ok)
    t.ok(keyset_root_address.Ok)
    console.log("(((((((((((((((((())))))))))))))))))");
    console.log("-------------------->",keyset_root_address);
    console.log("(((((((((((((((((())))))))))))))))))");
// Check if the key exist for the key
// This is befor this is created
    const checking_key_1 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_SIG_KEY_1})
    t.deepEqual(checking_key_1.Ok,"Doesn't Exists" )

// Lets create an agent key
    const key_commit = await liza.callSync('dpki_happ', "dpki", "create_agent_key", {
      agent_name:"MY_AGENT"
    })
    t.deepEqual(key_commit.Ok,null)


    const all_keys = await liza.callSync('dpki_happ', "dpki", "get_all_keys", {})
    console.log(all_keys);
    t.deepEqual(all_keys.Ok.length,2 )


/*
Check if the keys exist for the key
 Now it should exist
*/

  // Checking Agents initial Signing key
    const checking_key_2 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_SIG_KEY_1})
    t.deepEqual(checking_key_2.Ok,"live" )

  // Ceecking Agents initial Encryption key
    const checking_key_3 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_ENC_KEY_1})
    t.deepEqual(checking_key_3.Ok,"live" )

// Lets Update the keys just created
    const updated_key = await liza.callSync('dpki_happ', "dpki", "update_key", {
      old_key:AGENT_SIG_KEY_1,
      signed_old_key:SIGNED_AGENT_SIG_KEY_1_BY_REV_KEY,
      context:"NEWAGENT"
    })
    console.log("Updated Key: ",updated_key);
    t.deepEqual(updated_key.Ok,null)

    sleep.sleep(5)

// Check if the key exist for the key
// Now the old key should be shown as updated and the new should be live
    const checking_key_4 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_SIG_KEY_1})
    t.deepEqual(checking_key_4.Ok,"modified" )

    const checking_key_5 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_SIG_KEY_2})
    t.deepEqual(checking_key_5.Ok,"live" )


    const deleated_key = await liza.callSync('dpki_happ', "dpki", "delete_key", {
      old_key:AGENT_ENC_KEY_1,
      signed_old_key:SIGNED_AGENT_ENC_KEY_1_BY_REV_KEY
    })
    console.log("deleated_key: ", deleated_key);
    t.equal(deleated_key.Ok,null)
    console.log(" Deleated Key Succesfully ");

    sleep.sleep(5)

    const checking_key_6 = await liza.callSync('dpki_happ', "dpki", "key_status", {key:AGENT_ENC_KEY_1})
    t.deepEqual(checking_key_6.Ok,"deleted" )

    await liza.kill()
  })
}
