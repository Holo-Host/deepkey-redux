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
use crate::key_anchor::KeyAnchor;

pub fn handle_create_authorizor(authorization_key: HashString) -> ZomeApiResult<Address> {
    let revocation_authority = rules::handlers::handle_get_my_rules()?;
    let authorizor = Authorizor {
        authorization_key: authorization_key.clone(),
        revocation_authority,
        revocation_sig: Signature::from("TODO"),
    };
    let authorizor_entry = Entry::App("authorizor".into(), authorizor.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : authorization_key
    }.into());

    // Hopfully we bundle this two commits once we have that feature
    let address = hdk::commit_entry(&authorizor_entry)?;
    hdk::commit_entry(&key_anchor)?;

    Ok(address)
}

// pub fn handle_get_authorizor(address: Address) -> ZomeApiResult<Option<Entry>> {
//     hdk::get_entry(&address)
// }
