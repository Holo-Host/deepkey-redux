use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

use crate::key_registration;

pub fn handle_create_key_registration(payload: key_registration::KeyRegistration) -> ZomeApiResult<Address> {
    let entry = Entry::App("key_registration".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_key_registration(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
