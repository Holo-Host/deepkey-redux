use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    signature::Signature,
};

pub mod device_authorization;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuthorization {
    pub trusted_device_deepkey_agent_id1: HashString,
    pub trusted_device_deepkey_agent_id2: HashString,
    pub authorizor_id1_sig_of_xor: Signature,
    pub authorizor_id2_sig_of_xor: Signature,
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "device_authorization",
        description: "Struct that proves a sucessfull handshake between two devices",
        sharing: Sharing::Public,
        native_type: DeviceAuthorization,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_da: DeviceAuthorization, _validation_data: hdk::ValidationData| {
            {
                Ok(())
            }
        },

        links: []
    )
}
