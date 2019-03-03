use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    chain_header::ChainHeader,
    entry::Entry,
    error::HolochainError,
    hash::HashString,
    signature::Signature,
};
use hdk::{
    holochain_wasm_utils::api_serialization::{
            query::{
                QueryArgsOptions, QueryResult,
            },
    }
};

use crate::rules;
use crate::keyset_root;

pub fn handle_create_rules(revocation_key: HashString) -> ZomeApiResult<Address> {
    let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;
    hdk::debug(&keyset_root);
    let rule = rules::Rules{
        keyset_root:keyset_root.clone(),
        revocation_key,
        prior_revocation_self_sig:Signature::from("TODO")
    };
    let entry = Entry::App("rules".into(), rule.into());
    utils::commit_and_link(&entry, &keyset_root, "rules_link_tag")
}

// // TODO: Better return type
pub fn handle_get_rules() -> ZomeApiResult<Vec<Entry>> {
    let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;
    let r = hdk::get_links_and_load(&keyset_root,"rules_link_tag")?;
    Ok(r.iter()
        .map(|maybe_entry|{
        match maybe_entry {
            Ok(entry)=>{
                Ok(entry.to_owned())
            }
            _=>Err(ZomeApiError::Internal(
				"get_links did not return an app entry".to_string())
            )
        }
        })
        .filter_map(Result::ok)
        .collect()
    )
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
    Ok(address[0].to_owned())
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
