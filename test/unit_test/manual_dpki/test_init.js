const { simple_conductor_config } = require('../../config')

module.exports = (scenario) => {
  scenario("testing the init process", async(s, t) => {
    const { liza } = await s.players({
      liza: simple_conductor_config('liza'),
    }, true)

    await s.consistency()

    let c1 = await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi\",\"signed_auth_key\":\"CPhaw45L6MjxPOsVBFsTYkl35hS4h9yRNqsl1fqfNx5P6z6l6WE6aLSrBjD3Dfe3HSg3vNSHtC1QeN0FWBo+DQ==\"}"})
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

    await liza.kill()
  })
}
