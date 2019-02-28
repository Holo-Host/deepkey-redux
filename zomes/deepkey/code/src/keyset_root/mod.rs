use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    signature::Signature
};

pub mod keyset_root;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct KeysetRoot {
    pub first_deepkey_agent: HashString,
    pub root_pubkey: HashString,
    pub fda_signed_by_rootkey: Signature,
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "keyset_root",
        description: "Root hash that would be used as an anchor",
        sharing: Sharing::Public,
        native_type: KeysetRoot,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_rh: KeysetRoot, _validation_data: hdk::ValidationData| {
            {
                Ok(())
            }
        },

        links: []
    )
}
