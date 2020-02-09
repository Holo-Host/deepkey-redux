use crate::app_key::AppKey;
use hdk::holochain_persistence_api::hash::HashString;
use hdk::prelude::*;
use std::convert::TryFrom;

pub fn handle_register_key(
    app_dna_hash: String,
    app_name: String,
    public_key: HashString,
) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        "app_key".into(),
        AppKey::new(&app_dna_hash, &app_name, &public_key).into(),
    );
    hdk::commit_entry(&entry)
}

pub fn handle_get_registered_key() -> ZomeApiResult<Vec<AppKey>> {
    let raw_result: Vec<Result<AppKey, JsonError>> =
        query_local_chain_for_entry_type("app_key".to_string())?
            .iter()
            .map(|(_, entry)| match entry {
                Entry::App(_, data) => AppKey::try_from(data.to_owned()),
                _ => unreachable!(),
            })
            .collect();
    let mut result: Vec<AppKey> = Vec::new();
    for r in raw_result {
        result.push(r?);
    }
    Ok(result)
}

// ----------------------
// Helper functions
// ----------------------
pub fn query_local_chain_for_entry_type(
    entry_type: String,
) -> Result<Vec<(Address, Entry)>, HolochainError> {
    if let QueryResult::Entries(entries) = hdk::query_result(
        vec![entry_type].into(),
        QueryArgsOptions {
            entries: true,
            ..Default::default()
        },
    )? {
        Ok(entries)
    } else {
        Err(HolochainError::ErrorGeneric(
            "Unexpected hdk::query_result".to_string(),
        ))
    }
}
