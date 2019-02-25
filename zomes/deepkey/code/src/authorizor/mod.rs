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

// pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct Authorizor {
    pub authorization_key: HashString,
    pub revocation_sig: Signature, // (empty on Create, required on Update)
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "authorizor",
        description: "Used to set the authorizing keys for each device",
        sharing: Sharing::Public,
        native_type: Authorizor,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_kr: Authorizor, _validation_data: hdk::ValidationData| {
            {
                Ok(())
            }
        },

        links: []
    )
}
