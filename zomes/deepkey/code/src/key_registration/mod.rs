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
pub struct KeyRegistration {
    pub new_agent_key: HashString,
    pub authorization_sig: Signature,
    pub revocation_sig: Signature, //(missing on Create, required on Update or Delete)
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "key_registration",
        description: "Entry to register a any keys on DeepKey",
        sharing: Sharing::Public,
        native_type: KeyRegistration,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_kr: KeyRegistration, _validation_data: hdk::ValidationData| {
            {
                Ok(())
            }
        },

        links: []
    )
}
