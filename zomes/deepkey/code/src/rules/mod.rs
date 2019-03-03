use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    signature::Signature
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct Rules {
    pub keyset_root: HashString,
    pub revocation_key: HashString,
    pub prior_revocation_self_sig: Signature, //(Signed by RootKey on Create by RevKey on Updates)
}

// fn validationSource(source:HashString)->bool{
//     let keyset_root = keyset_root::handlers::handle_get_keyset_root()?;
//     if keyset_root.length === 0 return false
//     else return true
// }
pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "rules",
        description: "This is the rules that the agent sets for his DeepKey acc",
        sharing: Sharing::Public,
        native_type: Rules,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_r: Rules, _validation_data: hdk::ValidationData| {
            {
                // **Initial Validation**
                // Check that the origin is from a valid device
                // i.e. the agent is linked from RootHash
                // let source = &_validation_data.package.chain_header.provenances()[0].0;
                // if validationSource(source) {
                //     hdk::debug("Succesfully Validated that Source == first_deepkey_agent");
                //     Ok(())
                // }
                // else{
                //     Err("Could not Validate KeysetRoot: Source is not equal to the first_deepkey_agent".to_string())
                // }
                // **On Update**
                // Check if signed by Prior Revocation Key on Update
                // (field not required on Create)
                Ok(())
            }
        },

        links: [
            from!(
                "keyset_root",
                tag: "rules_link_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
