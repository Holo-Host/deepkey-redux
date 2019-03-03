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

                // **On Update**
                // Check if signed by Prior Revocation Key on Update
                // (field not required on Create)
                Ok(())
            }
        },

        links: [
            // from!(
            //     "keyset_root",
            //     tag: "rules_link_tag",
            //
            //     validation_package: || {
            //         hdk::ValidationPackageDefinition::Entry
            //     },
            //
            //     validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData| {
            //         Ok(())
            //     }
            // )
        ]
    )
}
