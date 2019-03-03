use hdk::{
    error::ZomeApiResult,
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    hash::HashString,
    signature::Signature,
};
use crate::keyset_root::KeysetRoot;

pub fn handle_set_keyset_root() -> ZomeApiResult<Address>   {

    let root : KeysetRoot = KeysetRoot {
        first_deepkey_agent: HashString::from(AGENT_ADDRESS.to_string()),
        root_pubkey: HashString::from(AGENT_ADDRESS.to_string()), // How to get the OTKey?
        fda_signed_by_rootkey: Signature::from("TODO"), // Need Sign Functions to sign the fda wit the rootkey
    };
    let entry = Entry::App("keyset_root".into(), root.into());

    utils::commit_and_link(&entry, &AGENT_ADDRESS, "deepkey_agent_link_tag")
}

pub fn handle_get_keyset_root() -> ZomeApiResult<utils::GetLinksLoadResult<KeysetRoot>> {
    utils::get_links_and_load_type(&AGENT_ADDRESS,"deepkey_agent_link_tag")
}
