const sleep = require('sleep');
async function genesis (agent){
  return await agent.call("dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})
}

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async(s, t, { liza, jack }) => {
    await genesis(liza)
    await genesis(jack)
    sleep.sleep(5)

    const jack_receives = await jack.call("dpki", "send_handshake_notify", {to:liza.agentId})
    console.log("jack_receives:: ",jack_receives);
    t.ok(jack_receives.Ok)

    const is_authorized = await liza.call("dpki", "authorize_device", {new_agent_hash: jack.agentId, new_agent_signed_xor: jack_receives.Ok })
    console.log("is_authorized:: ",is_authorized);
    t.deepEqual(is_authorized.Ok,null)

  })
}
