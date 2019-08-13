use hdk::{
    error::{ZomeApiResult,ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address,
        hash::HashString
    },
    holochain_core_types::{
        entry::Entry,
        signature::{Provenance, Signature},
    },
    AGENT_ADDRESS
};
use crate::device_authorization::DeviceAuthorization;
use crate::utils;


pub fn handle_authorize_device(new_agent_hash: HashString, new_agent_signed_xor: Signature) -> ZomeApiResult<()>{

    let xor: HashString = utils::get_xor_from_hashs(&AGENT_ADDRESS,&new_agent_hash);

    if !hdk::verify_signature(
        Provenance::new(
            new_agent_hash.to_owned(),
            new_agent_signed_xor.to_owned(),
        ),
        String::from(xor.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature of New Device Not Able to be Verified".to_string(),
        ));
    }

    let signed_xor = hdk::sign(xor.to_string()).map(Signature::from)?;
    let device_authorization = DeviceAuthorization {
        trusted_device_deepkey_agent_id1: HashString::from(AGENT_ADDRESS.to_string()),
        trusted_device_deepkey_agent_id2: new_agent_hash,
        authorizor_id1_sig_of_xor: signed_xor,
        authorizor_id2_sig_of_xor: new_agent_signed_xor,
    };
    commit_device_authorization(device_authorization)?;
    Ok(())

}

fn commit_device_authorization(payload: DeviceAuthorization) -> ZomeApiResult<Address> {
    let entry = Entry::App("device_authorization".into(), payload.into());
    hdk::commit_entry(&entry)
}
