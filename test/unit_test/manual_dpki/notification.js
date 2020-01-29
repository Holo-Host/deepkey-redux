const { simple_conductor_config, simple_2_conductor_config } = require('../../config')

async function liza_conductor_init (agent){
  return await agent.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi\",\"signed_auth_key\":\"CPhaw45L6MjxPOsVBFsTYkl35hS4h9yRNqsl1fqfNx5P6z6l6WE6aLSrBjD3Dfe3HSg3vNSHtC1QeN0FWBo+DQ==\"}"})
}

async function jack_conductor_init (agent){
  return await agent.callSync("dpki_happ", "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi\",\"signed_auth_key\":\"sCkN1Yqaxeso1JicovXBruDXtx20Sofa+I6A6xpe3LjJQ6zvqwrJ3qbbDgLmFIPqF5RAKWTSMI7BZr6+06k/DA==\"}"})
}

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async (s, t) => {
    const { liza, jack } = await s.players({ liza: simple_conductor_config("liza"), jack: simple_2_conductor_config("jack")}, true)

    await s.consistency()

    let a = await liza_conductor_init(liza)
    t.ok(a.Ok)
    a= await jack_conductor_init(jack)
    t.ok(a.Ok)

    const jack_receives = await jack.callSync("dpki_happ", "dpki", "send_handshake_notify", {to:liza.info('dpki_happ').agentAddress})
    console.log("jack_receives:: ",jack_receives);
    t.ok(jack_receives.Ok)

    const is_authorized = await liza.callSync("dpki_happ", "dpki", "authorize_device", {new_agent_hash: jack.info('dpki_happ').agentAddress, new_agent_signed_xor: jack_receives.Ok })
    console.log("is_authorized:: ",is_authorized);
    t.deepEqual(is_authorized.Ok,null)

    await liza.kill()
    await jack.kill()
  })
}
