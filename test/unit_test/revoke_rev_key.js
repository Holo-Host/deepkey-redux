const sleep = require('sleep');
const REVOCATION_KEY = "HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz";
const SIGNED_AUTH_KEY_1 ="LVeIAP0horN0UhEVuqZyDCPjcYzvQUj9AMRm4Hv+xtsS6QoHYUeudekZoVYcPtktf+tDTtP/yFu8O3+jsZDbBQ==";
const WRONG_SINGED_AUTH_KEY = "D16Dl3Cywos/AS/ANPqsvkRZCCKWPd1KTkdANOxqG1MXRtdCaTYYAOO13mcYYtfzWbaagwLk5oFlns2uQneUDg==";
const SIGNED_AUTH_KEY_2 ="LbEReAxFLkkzfOHRBixC7+DYKGao6lPBYsUycVg3NHmNx7p8237/9unBwrt/o+9P4IWkKR+QCYeFxqBNRnn+Dg==";
const NEW_REVOCATION_KEY = "HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz";
const REV_SIGNED_BY_REV_KEY = "1//sUXEZwizqTK6aefikiVQpXQaIbK224woC4DsSJ6U7B2Mx1GO6V9P7Du+rpPiuRVIbPJ9CGfz3vzbftp78AQ==";
const BAD_SIGNED_REV_KEY = "Jkz3AWHO5bEZ11OpsNeotTIr3CGH3wZcyqUAae+xEVy+MwXhrAS1lfzUbWSRQgsSMWUNSjYTtE9NNUHXPkQkBg=="
async function genesis (liza){
  return await liza.call("dpki", "init",  {params: "{\"revocation_key\": \"HcSCJy6gGntzgr95qrpddfqNPGp48mw36pq6m3333pDo8M8kPcmW89TdUM8iwmz\"}"})
}

module.exports = (scenario) => {
  scenario("testing the initial set up process and trying to update it", async(s, t, { liza }) => {

    await genesis(liza)

    sleep.sleep(5)

// Check if your getting the right hash
    const my_rules = await liza.call("dpki", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0]);
    t.ok(my_rules.Ok[0].entry.revocationKey,REVOCATION_KEY)

// The signature should not match and throw an error
    const err_on_commit = await liza.call("dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:BAD_SIGNED_REV_KEY})
    console.log(err_on_commit);
    t.ok(err_on_commit.Err )

    const sucessfull_commit = await liza.call("dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:REV_SIGNED_BY_REV_KEY})
    t.ok(sucessfull_commit.Ok )

    sleep.sleep(5);
// Check if your getting the right hash
    const my_updated_rules = await liza.call("dpki", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok[0]);
    t.deepEqual(my_updated_rules.Ok[0].entry.revocationKey,NEW_REVOCATION_KEY )


  })
}
