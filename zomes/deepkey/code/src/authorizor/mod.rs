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
pub struct Authorizor {
    pub authorization_key: HashString,
    pub revocation_authority: HashString,
    pub revocation_sig: Signature,
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
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                tag: "authorizor_link_tag",

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
