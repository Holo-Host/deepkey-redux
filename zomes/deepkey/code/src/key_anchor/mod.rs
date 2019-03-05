use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct KeyAnchor {
    pub pub_key: HashString,
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "key_anchor",
        description: "",
        sharing: Sharing::Public,
        native_type: KeyAnchor,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_r: KeyAnchor, _validation_data: hdk::ValidationData| {
            Ok(())
        },

        links: []
    )
}
