use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

use crate::root_hash;

pub fn handle_create_root_hash(payload: root_hash::RootHash) -> ZomeApiResult<Address> {
    let entry = Entry::App("root_hash".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_root_hash(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
