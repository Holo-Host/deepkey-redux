const { simple_conductor_config, handleHack } = require('../../config')

module.exports = (scenario) => {
  scenario("testing the init process", async(s, t) => {
    const { liza } = await s.players({
      liza: simple_conductor_config('liza'),
    }, true)

    // await liza.spawn(handleHack)
    // await s.consistency()
    let c1 = await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})

    console.log("INIT:: ",c1);

    t.ok(c1.Ok)

    let get_init_data = await liza.call('dpki_happ', "dpki", "get_initialization_data", {})

    console.log("Rules: ", get_init_data);

    t.ok(get_init_data.Ok)


    let get_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})

    console.log("Rules: ", get_rules);

    t.ok(get_rules.Ok)


    let get_auth = await liza.call('dpki_happ', "dpki", "get_authorizor", {})

    console.log("Auth: ", get_auth);

    t.ok(get_auth.Ok)

    // await liza.kill()
  })
}
