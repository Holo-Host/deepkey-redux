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

pub mod key_registration;

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
                /// **Initialize**
                // Check the Auhorizor linked to the AgentID if its has a valid authorization_sig
                // Rev Sig is Empty
                /// **Update Entry**
                // Validate Rev Sig
                Ok(())
            }
        },

        links: [
            from!(
                "%agent_id",
                tag: "key_registration_link_tag",

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
