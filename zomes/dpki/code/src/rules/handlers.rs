use hdk::{
    utils,
    error::{ZomeApiResult,ZomeApiError},
};
use hdk::holochain_core_types::{
    cas::content::Address,
    cas::content::AddressableContent,
    entry::Entry,
    hash::HashString,
    signature::Signature,
    link::LinkMatch,
};


use crate::rules::{
    self,
    Rules,
    GetResponse
};
use crate::keyset_root;



pub fn handle_create_rules(revocation_key: HashString) -> ZomeApiResult<Address> {
    // Checking if keyset_root Exists
    let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;
    // Checking if rules exists if they do then update the values
    let rules = handle_get_my_rule_details()?;

    if rules.len() == 0 {
        // create_new_rules(&keyset_root,&revocation_key)
        Err(ZomeApiError::from("Rules were not set during init".to_string()))
    } else {
        update_rules(&keyset_root,&revocation_key,rules[0].address.to_owned())
    }
}

pub fn create_new_rules(keyset_root:&HashString,revocation_key:&HashString, signature:Signature) -> ZomeApiResult<Address>{
    let rule = rules::Rules{
        keyset_root: keyset_root.clone(),
        revocation_key: revocation_key.to_owned(),
        prior_revocation_self_sig: signature
    };
    let entry = Entry::App("rules".into(), rule.into());
    utils::commit_and_link(&entry, &keyset_root, "rules_link_tag","")
}

fn update_rules(keyset_root:&HashString,revocation_key:&HashString,old_rule_address:HashString) -> ZomeApiResult<Address> {
    let rule = rules::Rules{
        keyset_root: keyset_root.clone(),
        revocation_key: revocation_key.to_owned(),
        prior_revocation_self_sig: Signature::from("TODO Updated")
    };
    let entry = Entry::App("rules".into(), rule.into());
    let address = hdk::update_entry(entry, &old_rule_address)?;
    // TODO: update the tag
    hdk::link_entries(&keyset_root,&address,"rules_link_tag","")?;
    Ok(address)
}

pub fn handle_get_my_rule_details() -> ZomeApiResult<Vec<GetResponse<rules::Rules>>> {
    let keyset_root = keyset_root::handlers::handle_get_my_keyset_root()?;

    Ok(
    hdk::utils::get_links_and_load_type(
        &keyset_root,
        LinkMatch::Exactly("rules_link_tag"), // the link type to match
        LinkMatch::Any
    )?.into_iter().map(|rules: Rules| {
        let address = Entry::App("rules".into(), rules.clone().into()).address();
        GetResponse{entry: rules, address}
    }).collect()
    )
}
