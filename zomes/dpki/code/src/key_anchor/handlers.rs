use hdk::prelude::*;
use hdk::{
    error::ZomeApiResult, holochain_core_types::entry::Entry, holochain_json_api::json::RawString,
    holochain_persistence_api::hash::HashString,
};

use crate::key_anchor::KeyAnchor;

pub fn handle_key_status(key: HashString) -> ZomeApiResult<RawString> {
    let key_anchor = Entry::App("key_anchor".into(), KeyAnchor { pub_key: key }.into());

    let key_anchor_address = hdk::entry_address(&key_anchor)?;

    if let GetEntryResultType::Single(result) = hdk::get_entry_result(
        &key_anchor_address,
        GetEntryOptions {
            status_request: StatusRequestKind::Initial,
            ..Default::default()
        },
    )?
    .result
    {
        match result.meta {
            Some(m) => Ok(RawString::from(String::from(m.crud_status))),
            None => Ok(RawString::from("Doesn't Exists".to_string())),
        }
    } else {
        Ok(RawString::from("Doesn't Exists"))
    }
}
