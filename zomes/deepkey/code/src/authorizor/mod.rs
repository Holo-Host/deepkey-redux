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
    validation::{EntryValidationData},
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
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<Authorizor>| {
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
        links: [
            from!(
                "%agent_id",
                tag: "authorizor_link_tag",

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


pub fn auth_path_definitions() -> ValidatingEntryType{
    entry!(
        name: "auth_key_derivation_path",
        description: "private entry provides us future ability to regenerate AuthKey from Master Seed",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<u64>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_r,validation_data:_} =>
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
        }
    )
}
