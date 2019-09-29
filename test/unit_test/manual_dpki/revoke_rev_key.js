const { simple_conductor_config, handleHack } = require('../../config')

const REVOCATION_KEY = "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi";
const NEW_REVOCATION_KEY = "HcSciCmrxP4w5yefdxiSc3W5nY7ic9yzxS4vpeX3iPtnvu7db59FY4z7vj55mDz";
const REV_SIGNED_BY_REV_KEY = "Aul6kMS4K4rW7wpRiPC154zdDtXRc8ZQEj3wV3eOufOdbBK839045X8SzHoNI0VJVYLVB9YbL6gJ2goja/jxDw==";
const BAD_SIGNED_REV_KEY = "Jkz3AWHO5bEZ11OpsNeotTIr3CGH3wZcyqUAae+xEVy+MwXhrAS1lfzUbWSRQgsSMWUNSjYTtE9NNUHXPkQkBg=="

async function conductor_init (liza){
  return await liza.call('dpki_happ', "dpki", "init_dpki",  {params: "{\"revocation_key\": \"HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi\",\"signed_auth_key\":\"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA==\"}"})
}

module.exports = (scenario) => {
  scenario("testing the initial set up process and trying to update it", async(s, t) => {
    const { liza } = await s.players({ liza: simple_conductor_config('liza')})

    await liza.spawn(handleHack)

    await conductor_init(liza)

// Check if your getting the right hash
    const my_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("My Rules: ",my_rules.Ok[0]);
    t.ok(my_rules.Ok[0].entry.revocationKey,REVOCATION_KEY)

// The signature should not match and throw an error
    const err_on_commit = await liza.call('dpki_happ', "dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:BAD_SIGNED_REV_KEY})
    console.log(err_on_commit);
    t.ok(err_on_commit.Err )

    const sucessfull_commit = await liza.call('dpki_happ', "dpki", "update_rules", {revocation_key:NEW_REVOCATION_KEY, signed_old_revocation_key:REV_SIGNED_BY_REV_KEY})
    t.ok(sucessfull_commit.Ok )

// Check if your getting the right hash
    const my_updated_rules = await liza.call('dpki_happ', "dpki", "get_rules", {})
    console.log("My Updated Rules: ",my_updated_rules.Ok[0]);
    t.deepEqual(my_updated_rules.Ok[0].entry.revocationKey,NEW_REVOCATION_KEY )

    await liza.kill()
  })
}
