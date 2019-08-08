#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::signature::Signature,
    holochain_json_api::{
        error::JsonError,
        json::{JsonString, RawString},
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},
};

pub mod authorizor;
// pub mod device_authorization;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod rules;
use rules::GetResponse;
// pub mod test;

pub mod dpki_trait;

define_zome! {
    entries: [
        authorizor::definitions(),
        authorizor::auth_path_definitions(),
        // device_authorization::definitions(),
        key_anchor::definitions(),
        key_registration::definitions(),
        key_registration::meta_definitions(),
        keyset_root::definitions(),
        rules::definitions()
        // rules::rev_path_definitions()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
    // DPKI Trait functions
        init: {
            inputs: | params: String |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: dpki_trait::init
        }
        is_initialized: {
            inputs: | |,
            outputs: |result: ZomeApiResult<bool>|,
            handler: dpki_trait::is_initialized
        }
        create_agent_key: {
            inputs: | agent_name:String |,
            outputs: |result: ZomeApiResult<()>|,
            handler: dpki_trait::create_agent_keys
        }

    // Other Functions
        get_initialization_data: {
            inputs: | |,
            outputs: |result: ZomeApiResult<HashString>|,
            handler: keyset_root::handlers::handle_get_my_keyset_root
        }
        update_rules: {
            inputs: | revocation_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: rules::handlers::handle_updating_rules
        }
        get_rules: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<GetResponse<rules::Rules>>> |,
            handler: rules::handlers::handle_get_my_rule_details
        }
        // To generate Authorizor Key
        // Derivation pattern use is
        // For Auth Seed: 'auth_seed:0'
        // For Auth Key: 'auth_key:0'
        set_authorizor: {
            inputs: | authorization_key_path: u64, signed_auth_key:Signature |,
            outputs: |result: ZomeApiResult<HashString>|,
            handler: authorizor::handlers::handle_create_authorizor
        }
        get_authorizor: {
            inputs: | |,
            outputs: |result: ZomeApiResult<authorizor::Authorizor> |,
            handler: authorizor::handlers::handle_get_authorizor
        }
        update_key: {
            inputs: | old_key:HashString, signed_old_key:Signature, context:String |,
            outputs: |result: ZomeApiResult<()>|,
            handler: key_registration::handlers::update_key
        }
        delete_key: {
            inputs: | old_key:HashString, signed_old_key:Signature |,
            outputs: |result: ZomeApiResult<()>|,
            handler: key_registration::handlers::handle_delete_key_registration
        }
        get_all_keys: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<key_registration::KeyMeta>> |,
            handler: key_registration::handlers::get_all_keys
        }
        key_status: {
            inputs: | key: HashString |,
            outputs: |result: ZomeApiResult<RawString>|,
            handler: key_anchor::handlers::handle_key_status
        }
        // sign: {
        //     inputs: | |,
        //     outputs: |result: ZomeApiResult<Signature>|,
        //     handler: test::handle_sign_message
        // }
    ]

    traits: {
        hc_public [
        // sign,
        init,
        is_initialized,
        get_initialization_data,
        create_agent_key,
        update_rules,
        get_rules,
        set_authorizor,
        get_authorizor,
        get_all_keys,
        update_key,
        delete_key,
        key_status
        ]
    }
}
