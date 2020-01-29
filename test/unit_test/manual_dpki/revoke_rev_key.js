const { simple_conductor_config } = require('../../config')

const REVOCATION_KEY = "HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi";
const NEW_REVOCATION_KEY = "HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi";
const REV_SIGNED_BY_REV_KEY = "xLD9u9XLBqr295xacnsaS9MTRmDAAzIpzvIOrR+2wvDOWezD6aFyEGsUoml/MKKga1i718uVmwL//Rze300CAA==";
const BAD_SIGNED_REV_KEY = "Jkz3AWHO5bEZ11OpsNeotTIr3CGH3wZcyqUAae+xEVy+MwXhrAS1lfzUbWSRQgsSMWUNSjYTtE9NNUHXPkQkBg=="

async function conductor_init (liza){
  return await liza.callSync('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCIgvyd46Q4d9xa4gesx8j5tE7crna8m9U4Z63yzmf5aob6t3mKTNIp8mp8fi\",\"signed_auth_key\":\"CPhaw45L6MjxPOsVBFsTYkl35hS4h9yRNqsl1fqfNx5P6z6l6WE6aLSrBjD3Dfe3HSg3vNSHtC1QeN0FWBo+DQ==\"}"})
}

module.exports = (scenario) => {
  scenario("testing the initial set up process and trying to update it", async(s, t) => {
    const { liza } = await s.players({ liza: simple_conductor_config('liza')}, true)

    await s.consistency()

    await conductor_init(liza)

// Check if your getting the right hash
    const my_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0]);
    t.ok(my_rules.Ok[0].entry.revocationKey,REVOCATION_KEY)

// The signature should not match and throw an error
    const err_on_commit = await liza.callSync('dpki_happ', "dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:BAD_SIGNED_REV_KEY})
    console.log(err_on_commit);
    t.ok(err_on_commit.Err )

    const sucessfull_commit = await liza.callSync('dpki_happ', "dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:REV_SIGNED_BY_REV_KEY})
    t.ok(sucessfull_commit.Ok )

// Check if your getting the right hash
    const my_updated_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok[0]);
    t.deepEqual(my_updated_rules.Ok[0].entry.revocationKey,NEW_REVOCATION_KEY )

    await liza.kill()
  })
}
