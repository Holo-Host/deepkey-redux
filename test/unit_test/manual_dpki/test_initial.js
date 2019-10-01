const { simple_conductor_config, handleHack } = require('../../config')
const { sleep }  = require('sleep')

async function conductor_init (liza){
  return await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
}

module.exports = (scenario) => {

  // scenario("testing checks if entries have been pushed", async(s, t) => {
  //   const { liza } = await s.players({ liza: simple_conductor_config('liza')})
  //   await liza.spawn(handleHack)
  //   // On conductor_init we have to make this call
  //   let address = await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
  //   sleep(5)
  //   let address_recheck = await await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
  //   t.ok(address.Ok)
  //   t.ok(address_recheck.Err)
  //   await liza.kill()
  // })

  scenario("testing to check if the DNA is initialized", async(s, t) => {
    const { liza } = await s.players({ liza: simple_conductor_config('liza')})
    await liza.spawn(handleHack)

    let check = await liza.callSync('dpki_happ', "dpki", "is_initialized", {})
    console.log("IS INITIALIZED: ",check);
    t.notOk(check.Ok)

    let address = await conductor_init(liza)

    check = await liza.callSync('dpki_happ', "dpki", "is_initialized", {})
    console.log("IS INITIALIZED: ",check);
    t.ok(check.Ok)
  })

  scenario("testing if create rules before the keyset_root should throw an error", async(s, t) => {
    const { liza } = await s.players({ liza: simple_conductor_config('liza')})

    await liza.spawn(handleHack)

  // This is to just test out if we get the right keyset_root address
    const keyset_root_address = await liza.callSync('dpki_happ', "dpki", "get_initialization_data", {})
    console.log("My KeysetRoot Address: ",keyset_root_address);
    t.deepEqual(keyset_root_address.Err.Internal,  'fn handle_get_my_keyset_root(): No KeysetRoot Exists' )


    await liza.kill()
  })
}
