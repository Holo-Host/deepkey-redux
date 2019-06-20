const sleep = require('sleep');
const REVOCATION_KEY = "HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa";
const SIGNED_AUTH_KEY_1 ="LVeIAP0horN0UhEVuqZyDCPjcYzvQUj9AMRm4Hv+xtsS6QoHYUeudekZoVYcPtktf+tDTtP/yFu8O3+jsZDbBQ==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";

const APP_KEY_1 = "HcSCJw6d7h53IAh8twROoUTe8qEiibgfxd3AuB9TwU7UktskXWiSyJ6b8334Umz";
const APP_KEY_2 = "HcScIidm755H3Oohd6PFA5TY9ebhxofqpbtZVceQ3yp4p6bbDfaGijB3sbapmii";
const SIGNED_APP_KEY_1_BY_REV_KEY = "b9VltsBRq71nPcJO/EzBz4EtUkqVPNhbS9ggYi90/hldNgHMOETtW19TdLxUXg3VpznjDP6pesyoBpcvzJXsBA==";
const SIGNED_APP_KEY_2_BY_REV_KEY = "88drLZ676Wez6SFSDmQrw1W0Cg4E04AdYWXfrgu6NFpr81NNsyoNr9jKU6StS/BaAVxR5mC9+cDAmgm+97pqCQ==";
module.exports = (scenario) => {
  scenario("testing out how genesis/init calls should be set up", async(s, t, { liza }) => {

    const keyset_root_address = await liza.call("dpki", "init", {revocation_key: REVOCATION_KEY})
    console.log("My keyset_root_address : ",keyset_root_address);
    t.ok(keyset_root_address.Ok)


    // // IGNORE : Using this func to get the revocation keys and auth key for my tests
    // let auth_signed_by_revocation_key = liza.call("converse","signed_auth_key",{key_id:1})
    // console.log("auth_signed_by_revocation_key: ",auth_signed_by_revocation_key);
    // t.ok(auth_signed_by_revocation_key.Ok)

    //***************
    // Solve this bug.
    // should work when you run this commented out test as well
    //***************
    // Failure to pass valid signature
    const wrong_rules = await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:WRONG_SINGED_AUTH_KEY
    })
    console.log("Error to set rules : ",wrong_rules);
    t.ok(wrong_rules.Err)


    // Setting the AUth
    const setting_rules = await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:SIGNED_AUTH_KEY_1
    })
    console.log("These are the setting_rules : ",setting_rules);
    t.ok(setting_rules.Ok)

    const auth_meta_1 = await liza.callSync("dpki", "get_auth_meta", {})
    console.log("These are the auth_meta : ",auth_meta_1);
    t.equal(auth_meta_1.Ok,1)

    // Updating the AUth
    const updating_rules = await liza.call("dpki", "set_authorizor", {
      authorization_key_path: 2,
      signed_auth_key:SIGNED_AUTH_KEY_2
    })
    console.log("These are the updating_rules : ",updating_rules);
    t.ok(updating_rules.Ok)

    const auth_meta_2 = await liza.callSync("dpki", "get_auth_meta", {})
    console.log("These are the auth_meta : ",auth_meta_2);
    t.equal(auth_meta_2.Ok,2)

    // Register A Key

    const registering_app_key = await liza.call("dpki", "create_agent_key", {
      derivation_index:1,
      key_type:"AppSig",
      context:"dna12345"
    })
    console.log("These are the registering_app_key : ",registering_app_key);
    t.ok(registering_app_key.Ok)

    sleep.sleep(5);

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

    sleep.sleep(5);

    const deleated_key = await liza.call("dpki", "delete_key", {
      old_key:APP_KEY_2,
      signed_old_key:SIGNED_APP_KEY_2_BY_REV_KEY
    })
    t.equal(deleated_key.Ok,null)
    console.log(" Deleated Key Succesfully ");

    sleep.sleep(5);

    const checking_key_4 = await liza.call("dpki", "key_status", {key:APP_KEY_2})
    t.deepEqual(checking_key_4.Ok,"deleted" )

  })
}
