/// TODO:
/// > Implementation for the following functions is not inclueded for the closed alpha

/// We need a struct for a handshake that need to happen to Authorize a device.
/// This commit would we that the device will reach out to the other device that is owned by the same Agent,
/// generating this hash and commiting.
/// Which means that this device is authorized not just by the FDA but all the other device owned by the agent.

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address,
        hash::HashString,
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        error::HolochainError,
        signature::Signature,
        validation::{EntryValidationData},
    }
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
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<DeviceAuthorization>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_domain_name,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                }

            }

        },
        links: []
    )
}
