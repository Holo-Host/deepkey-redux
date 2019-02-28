use hdk::{
    error::ZomeApiResult,
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
    error::HolochainError,
    hash::HashString,
    signature::Signature,
};
use std::error::Error;

use crate::keyset_root::KeysetRoot;



pub fn handle_create_keyset_root() -> ZomeApiResult<Address>   {

    let root : KeysetRoot = KeysetRoot {
        first_deepkey_agent: HashString::from(AGENT_ADDRESS.to_string()),
        root_pubkey: HashString::from(AGENT_ADDRESS.to_string()), // How to get the OTKey?
        fda_signed_by_rootkey: Signature::from(""), // Need Sign Functions to sign the fda wit the rootkey
    };

    let entry = Entry::App("keyset_root".into(), root.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_keyset_root(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry_initial(&address)
}
