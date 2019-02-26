use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

use crate::rules;

pub fn handle_create_rules(payload: rules::Rules) -> ZomeApiResult<Address> {
    let entry = Entry::App("rules".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_rules(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
