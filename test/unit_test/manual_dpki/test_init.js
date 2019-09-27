const { simple_conductor_config, handleHack } = require('../../config')

module.exports = (scenario) => {
  scenario("testing the notification to device handshaking", async(s, t) => {
    const { liza, jack } = await s.players({ liza: simple_conductor_config('liza'), jack: simple_conductor_config('jack')})

    await liza.spawn(handleHack)
    await jack.spawn(handleHack)

    let c1 = await liza.call('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})

    console.log("Chrck:: ",c1);

    t.ok(c1.Ok)

    let c2 = await jack.call('dpki_happ', "dpki", "init_dpki",  {params: "{\"first_deepkey_agent\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})

    console.log("Chrck:: ",c2);

    t.ok(c2.Ok)

    await liza.kill()
    await jack.kill()
  })
}
