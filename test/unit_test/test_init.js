const sleep = require('sleep');

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async(s, t, { liza, jack}) => {

    let c1 = await liza.call("dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})

    console.log("Chrck:: ",c1);

    t.ok(c1.Ok)

    let c2 = await jack.call("dpki", "init_dpki",  {params: "{\"first_deepkey_agent\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})

    console.log("Chrck:: ",c2);

    t.ok(c2.Ok)

  })
}
