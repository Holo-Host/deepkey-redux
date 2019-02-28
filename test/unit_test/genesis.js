
function genesis (liza){
  return liza.call("deepkey", "create_keyset_root", {})
}

module.exports = (scenario) => {
  scenario.runTape("testing out how genesis/init calls should be set up", (t, { liza }) => {
    const addr = genesis(liza)
    const result = liza.call("deepkey", "get_keyset_root", {"address": addr.Ok})
    t.deepEqual(result.Ok.App[0],"keyset_root" )
  })
}
