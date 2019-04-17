const test = require('tape');
const { Conductor, DnaInstance } = require('../../holochain-rust/nodejs_conductor')


const toml = `
[[agents]]
id = "liza_id"
name = "Li-za"
public_address = "HcSCJcJivPG673krt4eQQ77ojC7Qomo9bkgqEqpPaz553jcpnWoVQujgr4krjtz"
keystore_file = "/home/zo-el/.config/holochain/keys/HcSCJ4iA73hcM5Byi3vRq6t7sHj3Hh7jiG34sibNrEDx7q9eGYS3BGJJr5tbqtr"

# DNA and Instance of the HApp Store
[[dnas]]
id = "deepkey_dna"
file = "dist/DeepKey.dna.json"
hash = "Qm328wyq38924x"

[[instances]]
id = "deepkey_instance"
dna = "deepkey_dna"
agent = "liza_id"
[instances.logger]
type = "simple"
file = "app_spec2.log"
[instances.storage]
type = "file"
path = "tmp-storage-1"

[[interfaces]]
id = "websocket_interface"
[interfaces.driver]
type = "websocket"
port = 7777
[[interfaces.instances]]
id = "deepkey_instance"

[dpki]
instance_id = "deepkey_instance"
init_params = "{}"

`


test('Initial test (run)', t => {
    Conductor.run(toml, (stop, conductor) => {

      const instance = new DnaInstance('deepkey_instance', conductor)
      const keyset_root_address = instance.call("deepkey", "init", {})
      // console.log("keyset_root->",keyset_root_address);
      t.ok(keyset_root_address.Ok)

      const get_root_address = instance.call("deepkey", "get_initialization_data", {})
      // console.log("->",get_root_address);
      t.ok(get_root_address.Ok)

      stop()
      t.end()
    })
})
