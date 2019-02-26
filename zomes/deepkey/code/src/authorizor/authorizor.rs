use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

use crate::authorizor;

pub fn handle_create_authorizor(payload: authorizor::Authorizor) -> ZomeApiResult<Address> {
    let entry = Entry::App("authorizor".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_authorizor(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
