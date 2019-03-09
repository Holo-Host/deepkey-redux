use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    AGENT_ADDRESS,
    holochain_wasm_utils::api_serialization::{
        query::{
            QueryArgsOptions, QueryResult,
        },
    }
};
use hdk::holochain_core_types::{
    cas::content::Address,
    chain_header::ChainHeader,
    entry::Entry,
    error::HolochainError,
    hash::HashString,
    signature::Signature,
};
use core::convert::TryFrom;

use crate::authorizor::Authorizor;
use crate::rules;
use crate::key_anchor::KeyAnchor;

pub fn handle_create_authorizor(authorization_key:HashString) -> ZomeApiResult<Address> {
    let revocation_authority = rules::handlers::handle_get_my_rules()?;

    match handle_get_authorizor(){
        Ok(authorizor_entry)=>{
            match authorizor_entry{
                Some(authorizor_entry)=>{
                    match authorizor_entry{
                        Entry::App(_,value) =>{
                            let old_auth = Authorizor::try_from(value.to_owned())?;
                            update_authorizor(&authorization_key,&revocation_authority,old_auth)
                        },
                        _=>Err(ZomeApiError::from("handle_create_authorizor: Rules entry not found while updating".to_string()))
                    }
                },
                _=>Err(ZomeApiError::from("handle_create_authorizor: Rules entry not found while updating".to_string()))
            }
        },
        Err(_)=>{
            create_new_authorizor(&authorization_key,&revocation_authority)
        }
    }
}

fn create_new_authorizor(authorization_key: &HashString, revocation_authority: &HashString) -> ZomeApiResult<Address> {
    // Delete the old entry
    // add new entry
    let authorizor = Authorizor {
        authorization_key: authorization_key.to_owned(),
        revocation_authority:revocation_authority.to_owned(),
        revocation_sig: Signature::from("TODO"),
    };
    let authorizor_entry = Entry::App("authorizor".into(), authorizor.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : authorization_key.to_owned()
    }.into());

    // Hopfully we bundle this two commits once we have that feature
    match hdk::commit_entry(&authorizor_entry){
        Ok(address) => {
            hdk::commit_entry(&key_anchor)?;
            Ok(address)
        },
        Err(e)=>{
            Err(e)
        }
    }
}


fn update_authorizor(authorization_key:&HashString,revocation_authority:&HashString,old_auth:Authorizor) -> ZomeApiResult<Address> {
    let authorizor = Authorizor {
        authorization_key: authorization_key.to_owned(),
        revocation_authority:revocation_authority.to_owned(),
        revocation_sig: Signature::from("TODO"),
    };
    let entry = Entry::App("authorizor".into(), authorizor.into());
    let old_authorizor_address = handle_get_my_authorizor()?;
    let new_key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : authorization_key.to_owned()
    }.into());

    let old_key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : old_auth.authorization_key.to_owned()
    }.into());

    let old_key_anchor_address = hdk::entry_address(&old_key_anchor)?;
    match hdk::update_entry(entry, &old_authorizor_address){
        Ok(address)=>{
            match hdk::remove_entry(&old_key_anchor_address){
                Ok(_)=>{
                    hdk::commit_entry(&new_key_anchor)?;
                    Ok(address)
                },
                Err(_)=>{
                    Err(ZomeApiError::from("update_authorizor: Unable to remove key anchor".to_string()))
                }
            }
        },
        Err(_)=>{
            Err(ZomeApiError::from("update_authorizor: Unable to Update Key".to_string()))
        }
    }
}

// pub fn handle_get_authorizor(address: Address) -> ZomeApiResult<Option<Entry>> {
//     hdk::get_entry(&address)
// }

// TODO: Better return type
pub fn handle_get_authorizor() -> ZomeApiResult<Option<Entry>> {
    let authorizor_address = handle_get_my_authorizor()?;
    hdk::get_entry(&authorizor_address)
    // utils::get_as_type(rules_address)
}

pub fn handle_get_my_authorizor()->ZomeApiResult<HashString>{
    let authorizor_list = get_all_authorizor()?;
    let mut address:Vec<HashString>=Vec::new();
    for k in authorizor_list {
        if &AGENT_ADDRESS.to_string() == &k.0.provenances()[0].0.to_string(){
            address.push(k.0.entry_address().to_owned());
        }
    }
    if !address.is_empty() {
        Ok(address[0].to_owned())
    }
    else{
        Err(ZomeApiError::from("handle_get_my_authorizor: No Rules Exists".to_string()))
    }
}

// Example o/p
// {"entry_type":{"App":"authorizor"},"entry_address":"QmPZ1u6KezJBcup1siw9dUJ6hAgqix2DxjJzNPUb3Mpj1G",
// "provenances":[["liza------------------------------------------------------------------------------AAAOKtP2nI","TODO"]],
// "link":"QmSdoZMyqJFL7bBfsMP6wZYSmVd1kVqpoGrHuyRuxfqG7Y",
// "link_same_type":null,"link_crud":null,"timestamp":"1970-01-01T00:00:00+00:00"}'
pub fn get_all_authorizor() -> Result<Vec<(ChainHeader,Entry)>,HolochainError> {
    if let QueryResult::HeadersWithEntries( entries_with_headers ) = hdk::query_result(
        vec![
            "authorizor",
        ].into(),
        QueryArgsOptions{ headers: true, entries: true, ..Default::default()})? {
        Ok(entries_with_headers)
    } else {
        Err(HolochainError::ErrorGeneric( format!("Unexpected hdk::query_result")))
    }
}
