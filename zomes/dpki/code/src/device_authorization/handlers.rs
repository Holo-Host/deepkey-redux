use crate::device_authorization::DeviceAuthorization;
use crate::utils;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::Entry,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},
    AGENT_ADDRESS,
};

use crate::keyset_root::handlers::handle_get_my_keyset_root;

pub fn handle_authorize_device(
    new_agent_hash: HashString,
    new_agent_signed_xor: Signature,
) -> ZomeApiResult<()> {
    let xor: HashString = utils::get_xor_from_hashs(&AGENT_ADDRESS, &new_agent_hash);

    if !hdk::verify_signature(
        Provenance::new(new_agent_hash.to_owned(), new_agent_signed_xor.to_owned()),
        String::from(xor.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature of New Device Not Able to be Verified".to_string(),
        ));
    }

    let signed_xor = hdk::sign(xor.to_string()).map(Signature::from)?;
    let device_authorization = DeviceAuthorization {
        trusted_device_deepkey_agent_id1: HashString::from(AGENT_ADDRESS.to_string()),
        trusted_device_deepkey_agent_id2: new_agent_hash.to_owned(),
        authorizor_id1_sig_of_xor: signed_xor,
        authorizor_id2_sig_of_xor: new_agent_signed_xor,
    };
    let device_authorization_hash = commit_device_authorization(device_authorization)?;

    link_device_auth_to_root(&device_authorization_hash)?;

    link_new_agent_to_root(&new_agent_hash)?;

    Ok(())
}

fn commit_device_authorization(payload: DeviceAuthorization) -> ZomeApiResult<Address> {
    let entry = Entry::App("device_authorization".into(), payload.into());
    hdk::commit_entry(&entry)
}

fn link_new_agent_to_root(new_agent_hash: &Address) -> ZomeApiResult<Address> {
    let keyset_root = handle_get_my_keyset_root()?;
    hdk::link_entries(&keyset_root, new_agent_hash, "agent_link_tag", "")
}

fn link_device_auth_to_root(device_authorization_hash: &Address) -> ZomeApiResult<Address> {
    let keyset_root = handle_get_my_keyset_root()?;
    hdk::link_entries(
        &keyset_root,
        device_authorization_hash,
        "deepkey_device_link",
        "",
    )
}
