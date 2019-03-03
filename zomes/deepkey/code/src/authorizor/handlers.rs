use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    hash::HashString,
    signature::Signature,
};

use crate::authorizor::Authorizor;
use crate::rules;

pub fn handle_create_authorizor(authorization_key: HashString) -> ZomeApiResult<Address> {
    let revocation_authority = rules::handlers::handle_get_my_rules()?;
    let authorizor = Authorizor {
        authorization_key,
        revocation_authority,
        revocation_sig: Signature::from("TODO"),
    };
    let entry = Entry::App("authorizor".into(), authorizor.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_authorizor(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
