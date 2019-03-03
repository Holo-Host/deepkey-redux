use hdk::{
    error::{ZomeApiResult, ZomeApiError}
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    hash::HashString,
    signature::Signature,
};

use crate::rules;
use crate::keyset_root;

pub fn handle_create_rules(revocation_key: HashString) -> ZomeApiResult<Address> {
    let keyset_root = keyset_root::handlers::handle_get_keyset_root()?;
    hdk::debug(&keyset_root[0].address);
    let rule = rules::Rules{
        keyset_root:keyset_root[0].address.clone(),
        revocation_key,
        prior_revocation_self_sig:Signature::from("TODO")
    };
    let entry = Entry::App("rules".into(), rule.into());
    utils::commit_and_link(&entry, &keyset_root[0].address, "rules_link_tag")
}

pub fn handle_get_rules() -> ZomeApiResult<Vec<Entry>> {
    let keyset_root = keyset_root::handlers::handle_get_keyset_root()?;
    hdk::debug(&keyset_root[0].address);
    // utils::get_links_and_load_type(&keyset_root[0].address,"rules_link_tag")
    let r = hdk::get_links_and_load(&keyset_root[0].address,"rules_link_tag")?;
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
//     let keyset_root = keyset_root::handlers::handle_get_keyset_root()?;
//     utils::get_links_and_load_type(&keyset_root[0].address,"rules_link_tag")
// }
