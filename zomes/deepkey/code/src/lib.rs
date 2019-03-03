#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
};

pub mod authorizor;
// pub mod device_authorization;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod rules;

define_zome! {
    entries: [
        authorizor::definitions(),
        // device_authorization::definitions(),
        key_anchor::definitions(),
        key_registration::definitions(),
        keyset_root::definitions(),
        rules::definitions()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        set_keyset_root: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: keyset_root::handlers::handle_set_keyset_root
        }
        get_my_keyset_root: {
            inputs: | |,
            outputs: |result: ZomeApiResult<HashString>|,
            handler: keyset_root::handlers::handle_get_my_keyset_root
        }
        set_rules: {
            inputs: | revocation_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: rules::handlers::handle_create_rules
        }
        get_rules: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Entry>> |,
            handler: rules::handlers::handle_get_rules
        }
        set_authorizor: {
            inputs: | authorization_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: authorizor::handlers::handle_create_authorizor
        }
    ]

    traits: {
        hc_public [
        set_keyset_root,
        get_my_keyset_root,
        set_rules,
        get_rules,
        set_authorizor
        ]
    }
}
