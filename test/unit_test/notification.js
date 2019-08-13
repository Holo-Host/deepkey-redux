const sleep = require('sleep');
async function genesis (agent){
  return await agent.call("dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})
}

module.exports = (scenario) => {
  scenario("testing the initial set up process and trying to update it", async(s, t, { liza, jack }) => {
    await genesis(liza)
    sleep.sleep(5)
    const lizas_xor_signature = await liza.call("dpki", "send_handshake_notify", {to:jack.agentId})

    t.ok(lizas_xor_signature.Ok)

    const is_authorized = await jack.call("dpki", "authorize_device", {new_agent_hash: liza.agentId, new_agent_signed_xor: lizas_xor_signature.Ok})
    t.deepEqual(is_authorized.Ok,null)

  })
}
