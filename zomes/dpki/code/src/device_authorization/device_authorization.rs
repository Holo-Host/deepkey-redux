use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_core_types::{
        entry::Entry,
    }
};
use crate::device_authorization;

pub fn handle_create_device_authorization(payload: device_authorization::DeviceAuthorization) -> ZomeApiResult<Address> {
    let entry = Entry::App("device_authorization".into(), payload.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_device_authorization(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
