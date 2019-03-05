use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
    signature::Signature,
    hash::HashString
};

use crate::key_registration::KeyRegistration;
use crate::key_anchor::KeyAnchor;

pub fn handle_create_key_registration(new_agent_key: HashString) -> ZomeApiResult<Address> {
    let key_registration = KeyRegistration {
        new_agent_key:new_agent_key.clone(),
        authorization_sig: Signature::from("TODO"),
        prior_key: None, // (missing on Create, required on Update)
        revocation_sig: None, // (missing on Create, required on Update or Delete)
    };
    let key_registration_entry = Entry::App("key_registration".into(), key_registration.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : new_agent_key
    }.into());

    // Hopfully we bundle this two commits once we have that feature
    match hdk::commit_entry(&key_registration_entry){
        Ok(address) => {
            hdk::commit_entry(&key_anchor)?;
            Ok(address)
        },
        Err(e) => Err(e)
    }
}

// pub fn handle_get_key_registration(address: Address) -> ZomeApiResult<Option<Entry>> {
//     hdk::get_entry(&address)
// }
