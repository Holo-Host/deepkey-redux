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
pub struct KeysetRoot {
    pub first_deepkey_agent: HashString,
    pub root_pubkey: HashString,
    pub fda_signed_by_rootkey: Signature,
}

impl KeysetRoot {
    pub fn new(first_deepkey_agent: &HashString, root_pubkey: &HashString, fda_signed_by_rootkey:&Signature ) -> KeysetRoot {
        KeysetRoot {
            first_deepkey_agent: first_deepkey_agent.to_owned(),
            root_pubkey: root_pubkey.to_owned(),
            fda_signed_by_rootkey: fda_signed_by_rootkey.to_owned(),
        }
    }
}
pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "keyset_root",
        description: "Root hash that would be used as an anchor",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<KeysetRoot>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_kh,validation_data} =>
                {
                    // Validating if the source of the keyset_root is the same as the first_deepkey_agent
                    let source = &validation_data.package.chain_header.provenances()[0].0;
                    if &_kh.first_deepkey_agent == source {
                        hdk::debug("Succesfully Validated that Source == first_deepkey_agent")?;
                        Ok(())
                    }
                    else{
                        Err("Could not Validate KeysetRoot: Source is not equal to the first_deepkey_agent".to_string())
                    }
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
            to!(
                "%agent_id",
                tag: "deepkey_agent_link_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!(
                "rules",
                tag: "rules_link_tag",

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
