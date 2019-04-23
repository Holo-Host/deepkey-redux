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
pub struct KeyRegistration {
    pub new_agent_key: HashString,
    pub authorization_sig: Signature,
    pub prior_key: Option<HashString>, // (missing on Create, required on Update)
    pub revocation_sig: Option<Signature>, // (missing on Create, required on Update or Delete)
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "key_registration",
        description: "Entry to register a any keys on DeepKey",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<KeyRegistration>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_domain_name,validation_data:_} =>
                {
                    // **Initialize**
                    // Check the Auhorizor linked to the AgentID if its has a valid authorization_sig
                    // Rev Sig is Empty
                    // **Update Entry**
                    // Validate Rev Sig
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
        links: [ ]
    )
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
enum KeyType {
    AppUI,
    AppSig,
    AppEnc,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct KeyMeta {
    pub new_agent_key: HashString,
    pub derivation_index: String,
    pub key_type: KeyType,
    pub context: HashString // some_app_DNA_hash
}
pub fn meta_definitions() -> ValidatingEntryType{
    entry!(
        name: "key_meta",
        description: "private entry for NewKey registration which provides context and ability to regenerate from Master Seed.",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<KeyMeta>| {
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
        links: [ ]
    )
}
