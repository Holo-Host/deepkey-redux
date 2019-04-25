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
    error::HolochainError,
    json::{JsonString,RawString},
    hash::HashString,
    signature::Signature,
};

pub mod authorizor;
// pub mod device_authorization;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod rules;
pub mod dpki;

define_zome! {
    entries: [
        authorizor::definitions(),
        // authorizor::auth_path_definitions(),
        // device_authorization::definitions(),
        key_anchor::definitions(),
        key_registration::definitions(),
        // key_registration::meta_definitions(),
        keyset_root::definitions(),
        rules::definitions()
        // rules::rev_path_definitions()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        init: {
            inputs: | revocation_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: dpki::init
        }
        get_initialization_data: {
            inputs: | |,
            outputs: |result: ZomeApiResult<HashString>|,
            handler: keyset_root::handlers::handle_get_my_keyset_root
        }
        update_rules: {
            inputs: | revocation_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: rules::handlers::handle_create_rules
        }
        get_rules: {
            inputs: | |,
            outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<rules::Rules>> |,
            handler: rules::handlers::handle_get_my_rule_details
        }
        set_authorizor: {
            inputs: | authorization_key_path: u64, signed_auth_key:Signature |,
            outputs: |result: ZomeApiResult<HashString>|,
            handler: authorizor::handlers::handle_create_authorizor
        }
        set_key: {
            inputs: | new_agent_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: key_registration::handlers::handle_create_key_registration
        }
        key_status: {
            inputs: | key: HashString |,
            outputs: |result: ZomeApiResult<RawString>|,
            handler: key_anchor::handlers::handle_key_status
        }
    ]

    traits: {
        hc_public [
        init,
        get_initialization_data,
        update_rules,
        get_rules,
        set_authorizor,
        set_key,
        key_status
        ]
    }
}
