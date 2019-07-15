use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, entry::Entry, error::HolochainError, signature::Signature,
        validation::EntryValidationData,
    },
    holochain_json_api::{
        error::JsonError,
        json::{default_to_json, JsonString},
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},
};
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::Debug;

pub mod handlers;
use crate::keyset_root::KeysetRoot;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address,
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct Rules {
    pub keyset_root: HashString,
    pub revocation_key: HashString,
    pub prior_revocation_self_sig: Signature, //(Signed by RootKey on Create by RevKey on Updates)
}

fn validation_source(
    source: &HashString,
    keyset_root_address: HashString,
) -> Result<bool, HolochainError> {
    let keyset_roots_entry = hdk::get_entry(&keyset_root_address)?;
    if let Some(Entry::App(_, json_string)) = keyset_roots_entry {
        let root = KeysetRoot::try_from(json_string)?;
        if &root.first_deepkey_agent == source {
            return Ok(true);
        }
        Ok(false)
    } else {
        Ok(false)
    }
}
pub fn definitions() -> ValidatingEntryType {
    entry!(
        name: "rules",
        description: "This is the rules that the agent sets for his DeepKey acc",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<Rules>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_r,validation_data} =>
                {
                    // **Initial Validation**
                    // Check that the origin is from a valid device
                    // i.e. the agent is linked from RootHash
                    let source = &validation_data.package.chain_header.provenances()[0].0;
                    match validation_source(source,_r.keyset_root){
                        Ok(v)=>{
                            if v {return Ok(())}
                            else {return Err("Could not Validate Rules: Source is not equal to the provenances".to_string())}
                        }
                        _=> Err("Could not Validate Rules: Source is not equal to the provenances".to_string())
                    }
                    // **On Update**
                    // Check if signed by Prior Revocation Key on Update
                    // (field not required on Create)
                    // Ok(())
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
                link_type: "rules_link_tag",

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

pub fn rev_path_definitions() -> ValidatingEntryType {
    entry!(
        name: "rev_key_derivation_path",
        description: "private entry provides us future ability to regenerate RevKey from Master Seed",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<String>| {
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
