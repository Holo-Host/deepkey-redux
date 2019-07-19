const sleep = require('sleep');
const REVOCATION_KEY = "HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa";
const SIGNED_AUTH_KEY_1 ="LVeIAP0horN0UhEVuqZyDCPjcYzvQUj9AMRm4Hv+xtsS6QoHYUeudekZoVYcPtktf+tDTtP/yFu8O3+jsZDbBQ==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";
const AGENT_SIG_KEY_1 = "HcSci7FvFag6tnzhatmYaXz6Ab8Tijpvs7d6s69b3UH7WtYjmWGM7Q536WGf89i";
const SIGNED_AGENT_SIG_KEY_1_BY_REV_KEY ="b9VltsBRq71nPcJO/EzBz4EtUkqVPNhbS9ggYi90/hldNgHMOETtW19TdLxUXg3VpznjDP6pesyoBpcvzJXsBA==";
const AGENT_ENC_KEY_1 = "HcKciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SIGNED_AGENT_ENC_KEY_1_BY_REV_KEY = "M1XZQyfFdQYsZMBCQzs5Ham1s9jyFoMuvySs6+I46dPtV/+1NzOIvdhk1qphUjOWb2qIPnIJ+K3iL9NRnV5wCw==";
const APP_KEY_2 = "HcScIidm755H3Oohd6PFA5TY9ebhxofqpbtZVceQ3yp4p6bbDfaGijB3sbapmii";
async function conductor_init (liza){
  return await liza.app.call("dpki", "init",  {params: "{\"revocation_key\": \"HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa\"}"})
}

module.exports = (scenario) => {
  scenario("testing out how conductor should be set up", async(s, t, { liza }) => {
// On conductor_init we have to make this call
    let address = await conductor_init(liza)

// This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.app.call("dpki", "get_initialization_data", {})
    t.equal(keyset_root_address.Ok,address.Ok)

// Lets create an authorizor key
// QUESTION : How do we generate this auth_key and sign it (its signed using the deepkey agent key) ?
    const authorizor_commit =await liza.app.call("dpki", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:SIGNED_AUTH_KEY_1
    })
    console.log(authorizor_commit);
    t.ok(authorizor_commit.Ok)

// Check if the key exist for the authorizor
    const checking_authorizor_key = await liza.app.call("dpki", "key_status", {key:authorizor_commit.Ok})
    t.deepEqual(checking_authorizor_key.Ok,"live" )

// Check if the key exist for the key
// This is befor this is created
    const checking_key_1 = await liza.app.call("dpki", "key_status", {key:AGENT_SIG_KEY_1})
    t.deepEqual(checking_key_1.Ok,"Doesn't Exists" )

// Lets create an agent key
    const key_commit = await liza.app.call("dpki", "create_agent_key", {
      context:"MY_AGENT"
    })
    t.deepEqual(key_commit.Ok,null)

/*
Check if the keys exist for the key
 Now it should exist
*/

  // Checking Agents initial Signing key
    const checking_key_2 = await liza.app.call("dpki", "key_status", {key:AGENT_SIG_KEY_1})
    t.deepEqual(checking_key_2.Ok,"live" )

  // Ceecking Agents initial Encryption key
    const checking_key_3 = await liza.app.call("dpki", "key_status", {key:AGENT_ENC_KEY_1})
    t.deepEqual(checking_key_3.Ok,"live" )

// Lets Update the keys just created
    // const updated_key = await liza.app.call("dpki", "update_key", {
    //   old_key:AGENT_SIG_KEY_1,
    //   signed_old_key:SIGNED_APP_KEY_1_BY_REV_KEY,
    //   context:"MY_AGENT"
    // })
    // console.log("Updated Key: ",updated_key);
    // t.ok(updated_key.Ok)

// // Check if the key exist for the key
// // Now the old key should be shown as updated and the new should be live
//     const checking_key_3 = await liza.app.call("dpki", "key_status", {key:APP_KEY_1})
//     t.deepEqual(checking_key_3.Ok,"modified" )
//
//     const checking_key_4 = await liza.app.call("dpki", "key_status", {key:APP_KEY_2})
//     t.deepEqual(checking_key_4.Ok,"live" )
//

    const deleated_key = await liza.app.call("dpki", "delete_key", {
      old_key:AGENT_ENC_KEY_1,
      signed_old_key:SIGNED_AGENT_ENC_KEY_1_BY_REV_KEY
    })
    console.log("deleated_key: ", deleated_key);
    t.equal(deleated_key.Ok,null)
    console.log(" Deleated Key Succesfully ");

    const checking_key_5 = await liza.app.call("dpki", "key_status", {key:AGENT_ENC_KEY_1})
    t.deepEqual(checking_key_5.Ok,"deleted" )

  })
}

// module.exports = (scenario) => {
//   scenario("FakeData: This function is just for getting the signature back for testing", async(s, t, { liza }) => {
//     const signature = await liza.app.call("dpki","sign",{})
//     console.log("SIGNATURE:-> ",signature);
//   })
// }
