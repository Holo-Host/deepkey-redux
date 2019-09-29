/// We need a struct for a handshake that need to happen to Authorize a device.
/// This commit would we that the device will reach out to the other device that is owned by the same Agent,
/// generating this hash and commiting.
/// Which means that this device is authorized not just by the FDA but all the other device owned by the agent.
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        signature::{Provenance, Signature},
        validation::EntryValidationData,
    },
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::hash::HashString,
};

pub mod handlers;
use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuthorization {
    pub trusted_device_deepkey_agent_id1: HashString,
    pub trusted_device_deepkey_agent_id2: HashString,
    pub authorizor_id1_sig_of_xor: Signature,
    pub authorizor_id2_sig_of_xor: Signature,
}

pub fn definitions() -> ValidatingEntryType {
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
                EntryValidationData::Create{entry,validation_data} =>
                {
                    let source = &validation_data.package.chain_header.provenances()[0].0;

                    if &entry.trusted_device_deepkey_agent_id1 == source || &entry.trusted_device_deepkey_agent_id2 == source {
                        let xor = utils::get_xor_from_hashs(&entry.trusted_device_deepkey_agent_id1,&entry.trusted_device_deepkey_agent_id2);
                        if !hdk::verify_signature(
                            Provenance::new(
                                entry.trusted_device_deepkey_agent_id1.to_owned(),
                                entry.authorizor_id1_sig_of_xor.to_owned(),
                            ),
                            String::from(xor.to_owned()),
                        )? || !hdk::verify_signature(
                            Provenance::new(
                                entry.trusted_device_deepkey_agent_id2.to_owned(),
                                entry.authorizor_id2_sig_of_xor.to_owned(),
                            ),
                            String::from(xor.to_owned()),
                        )? {
                            return Err(
                                "Validation Error: Signature in DeviceAuthorization is invalid".to_string(),
                            );
                        }
                        return Ok(())
                    }
                    Err("Validation Error: DeviceAuthorization source is invalid ".to_string())

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
        links: [
            from!(
                "keyset_root",
                link_type: "deepkey_device_link",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
