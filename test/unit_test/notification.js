const sleep = require('sleep')
const { liza_conductor_config, jack_conductor_config } = require('../config')

// function genesis (agent){
//   return agent.call("dpki_happ", "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})
// }

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async (s, t) => {
    const { liza, jack } = await s.players({ liza: liza_conductor_config(), jack: jack_conductor_config()}, true)
    // await genesis(liza)
    // await genesis(jack)
    await s.consistency()

    const jack_receives = await jack.call("dpki_happ", "dpki", "send_handshake_notify", {to:liza.info('dpki_happ').agentAddress})
    console.log("jack_receives:: ",jack_receives);
    t.ok(jack_receives.Ok)

    const is_authorized = await liza.call("dpki_happ", "dpki", "authorize_device", {new_agent_hash: jack.info('dpki_happ').agentAddress, new_agent_signed_xor: jack_receives.Ok })
    console.log("is_authorized:: ",is_authorized);
    t.deepEqual(is_authorized.Ok,null)

  })
}
