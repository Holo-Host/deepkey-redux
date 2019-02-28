use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

use crate::keyset_root;

pub fn handle_create_keyset_root(payload: keyset_root::KeysetRoot) -> ZomeApiResult<Address> {
    let entry = Entry::App("keyset_root".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_keyset_root(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
