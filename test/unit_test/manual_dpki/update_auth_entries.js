const {
  simple_conductor_config
} = require('../../config')
const REVOCATION_KEY = "HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi";
// const SIGNED_AUTH_KEY_1 ="zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==";
// const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
// const SIGNED_AUTH_KEY_2 ="MG2L0DObZ+m/Zr4bWp/LRUD5FM5W/QZtYafxVEhehyujpPvGdgROCAApAIXl+UpQy1evDU+LnShZzY/emIbKDw==";

async function conductor_init(liza) {
  return await liza.callSync('dpki_happ', "dpki", "init_dpki", {
    params: "{\"revocation_key\": \"HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi\",\"signed_auth_key\":\"CPhaw45L6MjxPOsVBFsTYkl35hS4h9yRNqsl1fqfNx5P6z6l6WE6aLSrBjD3Dfe3HSg3vNSHtC1QeN0FWBo+DQ==\"}"
  })
}

module.exports = (scenario) => {

  scenario("testing checks if entries have been pushed", async (s, t) => {
    const {
      liza
    } = await s.players({
      liza: simple_conductor_config('liza')
    }, true)

    await s.consistency()
    // On conductor_init we have to make this call
    let address = await conductor_init(liza)
    let address_recheck = await conductor_init(liza)
    t.ok(address.Ok)
    t.ok(address_recheck.Err)
    await liza.kill()
  })

  scenario("testing to check if the DNA is initialized", async (s, t) => {
    const {
      liza
    } = await s.players({
      liza: simple_conductor_config('liza')
    }, true)

    await s.consistency()

    let check = await liza.call('dpki_happ', "dpki", "is_initialized", {})
    console.log("IS INITIALIZED: ", check);
    t.notOk(check.Ok)
    let address = await conductor_init(liza)
    check = await liza.call('dpki_happ', "dpki", "is_initialized", {})
    console.log("IS INITIALIZED: ", check);
    t.ok(check.Ok)
    await liza.kill()
  })

  scenario("testing if create rules before the keyset_root should throw an error", async (s, t) => {
    const {
      liza
    } = await s.players({
      liza: simple_conductor_config('liza')
    }, true)

    await s.consistency()

    // This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.call('dpki_happ', "dpki", "get_initialization_data", {})
    console.log("My KeysetRoot Address: ", keyset_root_address);
    t.deepEqual(keyset_root_address.Err.Internal, 'fn handle_get_my_keyset_root(): No KeysetRoot Exists')

    await liza.kill()
  })


  scenario("testing the initial set up process and trying to update it", async (s, t) => {
    const {
      liza
    } = await s.players({
      liza: simple_conductor_config('liza')
    }, true)

    await s.consistency()

    await conductor_init(liza)

    const check_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("Initial Rules: ", check_rules);
    t.deepEqual(check_rules.Ok.length, 1)

    // Check if your getting the right hash
    const my_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("My Rules: ", my_rules.Ok[0]);
    t.deepEqual(my_rules.Ok[0].entry.revocationKey, REVOCATION_KEY)

    // Lets create an authorizor key
    const authorizor_commit = await liza.call('dpki_happ', "dpki", "get_authorizor", {})
    t.ok(authorizor_commit.Ok)

    // Check if the key exist for the authorizor
    const not_registered_key = await liza.call('dpki_happ', "dpki", "key_status", {
      key: "Not-Registered-Key"
    })
    t.deepEqual(not_registered_key.Ok, "Doesn\'t Exists")

    // Check if the key exist for the authorizor
    const checking_authorizor_key = await liza.call('dpki_happ', "dpki", "key_status", {
      key: authorizor_commit.Ok.authorizationKey
    })
    t.deepEqual(checking_authorizor_key.Ok, "live")

    // TODO: Waiting for the Auth key gen process in CLI and HDK create the same keys
    // // Lets create an authorizor key
    //     const updated_authorizor_commit = await liza.callSync('dpki_happ', "dpki", "set_authorizor", {
    //       authorization_key_path: 2,
    //       signed_auth_key:SIGNED_AUTH_KEY_2
    //     })
    //     t.ok(updated_authorizor_commit.Ok)
    //
    //     const checking_new_authorizor_key = await liza.callSync('dpki_happ', "dpki", "key_status", {key:updated_authorizor_commit.Ok})
    //     t.deepEqual(checking_new_authorizor_key.Ok,"live" )
    //
    // // Check if the key exist for the authorizor
    //     const checking_old_authorizor_key = await liza.callSync('dpki_happ', "dpki", "key_status", {key:authorizor_commit.Ok})
    //     t.deepEqual(checking_old_authorizor_key.Ok,"modified" )

    await liza.kill()
  })
}