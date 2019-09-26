const { simple_conductor_config, simple_2_conductor_config, handleHack } = require('../config')
const { sleep } = require('sleep')

async function liza_conductor_init (agent){
  return await agent.call('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
}

async function jack_conductor_init (agent){
  return await agent.call("dpki_happ", "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCI7fRqt5wb7r6i46f5AeGW6zcNuq3i94fQVtFOPromhzoukr9DabcZqzxzir\",\"signed_auth_key\":\"bQNCtt9Xa7Ii4mCgOGSt8InVLA6HbrFjhYBoc4lDKMtxbY65kQoMNR/mHCuBq5rBYtyaZXG9Jpa9o8WD2eSrCw==\"}"})
}

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async (s, t) => {
    const { liza, jack } = await s.players({ liza: simple_conductor_config("liza"), jack: simple_2_conductor_config("jack")}, false)

    await liza.spawn(handleHack)
    await jack.spawn(handleHack)

    let a = await liza_conductor_init(liza)
    t.ok(a)
    a= await jack_conductor_init(jack)
    t.ok(a)

    const jack_receives = await jack.call("dpki_happ", "dpki", "send_handshake_notify", {to:liza.info('dpki_happ').agentAddress})
    console.log("jack_receives:: ",jack_receives);
    t.ok(jack_receives.Ok)

    sleep(5)
    await s.consistency()

    const is_authorized = await liza.call("dpki_happ", "dpki", "authorize_device", {new_agent_hash: jack.info('dpki_happ').agentAddress, new_agent_signed_xor: jack_receives.Ok })
    console.log("is_authorized:: ",is_authorized);
    t.deepEqual(is_authorized.Ok,null)

    await liza.kill()
    await jack.kill()
  })
}
