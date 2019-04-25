const sleep = require('sleep');

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", async(t, { liza }) => {
    const REVOCATION_KEY = "HcScIXuxtWI6ttc5gngvQTsDnHtynb5dzyDujh37mNo43nf7ZRB5UZKmR9953pa";
    const SIGNED_AUTH_KEY_1 ="D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAMN13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
    const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";

    const SIGNED_AUTH_KEY_2 ="eMdi7uIuCkrdfpyTR5hSXnlUOtVEYMRRoQgLvc9Qg531ywCW6M9Jg7GD+LdeRfpCE0bUhk3R1ChTBhy8C6yIAw==";

    const keyset_root_address = liza.call("deepkey", "init", {revocation_key: REVOCATION_KEY})
    console.log("My keyset_root_address : ",keyset_root_address.Ok);
    t.ok(keyset_root_address.Ok)


    // // IGNORE : Using this func to get the revocation keys and auth key for my tests
    // let auth_signed_by_revocation_key = liza.call("converse","signed_auth_key",{key_id:1})
    // console.log("auth_signed_by_revocation_key: ",auth_signed_by_revocation_key);
    // t.ok(auth_signed_by_revocation_key.Ok)
    // // t.equal(auth_signed_by_revocation_key.Ok,SIGNED_AUTH_KEY_1)

    //***************
    // Solve this bug.
    // should work when you run this commented out test as well
    //***************
    // // Failure to pass valid signature
    // const wrong_rules = liza.call("deepkey", "set_authorizor", {
    //   authorization_key_path: 1,
    //   signed_auth_key:WRONG_SINGED_AUTH_KEY
    // })
    // console.log("Error to set rules : ",wrong_rules);
    // t.ok(wrong_rules.Err)


    // Setting the AUth
    const setting_rules = await liza.callSync("deepkey", "set_authorizor", {
      authorization_key_path: 1,
      signed_auth_key:SIGNED_AUTH_KEY_1
    })
    console.log("These are the setting_rules : ",setting_rules);
    t.ok(setting_rules.Ok)

    // Updating the AUth
    const updating_rules = await liza.callSync("deepkey", "set_authorizor", {
      authorization_key_path: 2,
      signed_auth_key:SIGNED_AUTH_KEY_2
    })
    console.log("These are the updating_rules : ",updating_rules);
    t.ok(updating_rules.Ok)

  })
}
