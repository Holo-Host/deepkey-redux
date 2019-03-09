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

use crate::rules;
use crate::keyset_root;

pub fn handle_create_rules(revocation_key: HashString) -> ZomeApiResult<Address> {
    // Checking if keyset_root Exists
    let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;
    // Checking if rules exists if they do then update the values
    match handle_get_rules(){
        Ok(rules_entry)=>{
            match rules_entry{
                Some(rules_entry)=>{
                    match rules_entry{
                        Entry::App(_,value) =>{
                            let r = rules::Rules::try_from(value.to_owned())?;
                            update_rules(&keyset_root,&revocation_key,r.revocation_key)
                        },
                        _=>Err(ZomeApiError::from("handle_create_rules: Rules entry not found while updating".to_string()))
                    }
                },
                _=>Err(ZomeApiError::from("handle_create_rules: Rules entry not found while updating".to_string()))
            }
        },
        Err(_)=>{
            create_new_rules(&keyset_root,&revocation_key)
        }
    }
}

fn create_new_rules(keyset_root:&HashString,revocation_key:&HashString) -> ZomeApiResult<Address>{
    let rule = rules::Rules{
        keyset_root:keyset_root.clone(),
        revocation_key:revocation_key.to_owned(),
        prior_revocation_self_sig:Signature::from("TODO")
    };
    let entry = Entry::App("rules".into(), rule.into());
    utils::commit_and_link(&entry, &keyset_root, "rules_link_tag")
}

fn update_rules(keyset_root:&HashString,revocation_key:&HashString,_old_revocation:HashString) -> ZomeApiResult<Address> {
    let rule = rules::Rules{
        keyset_root:keyset_root.clone(),
        revocation_key:revocation_key.to_owned(),
        prior_revocation_self_sig:Signature::from("TODO Updated")
    };
    let entry = Entry::App("rules".into(), rule.into());
    let old_rule_address = handle_get_my_rules()?;
    let address = hdk::update_entry(entry, &old_rule_address)?;
    hdk::link_entries(&keyset_root,&address,"rules_link_tag")?;
    Ok(address)
}

// TODO: Better return type
pub fn handle_get_rules() -> ZomeApiResult<Option<Entry>> {
    let rules_address = handle_get_my_rules()?;
    hdk::get_entry(&rules_address)
    // utils::get_as_type(rules_address)
}

// TODO: not passign the uitls check
// pub fn handle_get_rules() -> ZomeApiResult<utils::GetLinksLoadResult<keyset_root::KeysetRoot>> {
//     let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;
//     utils::get_links_and_load_type(&keyset_root,"rules_link_tag")
// }

pub fn handle_get_my_rules()->ZomeApiResult<HashString>{
    let rules_list = get_all_rules()?;
    let mut address:Vec<HashString>=Vec::new();
    for k in rules_list {
        if &AGENT_ADDRESS.to_string() == &k.0.provenances()[0].0.to_string(){
            address.push(k.0.entry_address().to_owned());
        }
    }
    if !address.is_empty() {
        Ok(address[0].to_owned())
    }
    else{
        Err(ZomeApiError::from("handle_get_my_rules: No Rules Exists".to_string()))
    }
}

// Example o/p
// {"entry_type":{"App":"rules"},"entry_address":"QmPZ1u6KezJBcup1siw9dUJ6hAgqix2DxjJzNPUb3Mpj1G",
// "provenances":[["liza------------------------------------------------------------------------------AAAOKtP2nI","TODO"]],
// "link":"QmSdoZMyqJFL7bBfsMP6wZYSmVd1kVqpoGrHuyRuxfqG7Y",
// "link_same_type":null,"link_crud":null,"timestamp":"1970-01-01T00:00:00+00:00"}'
pub fn get_all_rules() -> Result<Vec<(ChainHeader,Entry)>,HolochainError> {
    if let QueryResult::HeadersWithEntries( entries_with_headers ) = hdk::query_result(
        vec![
            "rules",
        ].into(),
        QueryArgsOptions{ headers: true, entries: true, ..Default::default()})? {
        Ok(entries_with_headers)
    } else {
        Err(HolochainError::ErrorGeneric( format!("Unexpected hdk::query_result")))
    }
}
