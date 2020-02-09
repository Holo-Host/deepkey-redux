#[macro_use]
extern crate hdk;
use hdk::prelude::*;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    holochain_core_types::{
        signature::Signature,
    },
    holochain_json_api::{
        json::RawString,
    },
    holochain_persistence_api::hash::HashString,
};

mod app_key;
mod authorizor;
mod device_authorization;
mod dpki_trait;
mod key_anchor;
mod key_registration;
mod keyset_root;
mod rules;
mod utils;

use rules::GetResponse;

define_zome! {
    entries: [
        // Public Entires
        authorizor::definitions(),
        device_authorization::definitions(),
        key_anchor::definitions(),
        key_registration::definitions(),
        keyset_root::definitions(),
        rules::definitions(),
        // Private Entries
        key_registration::meta_definitions(),
        authorizor::auth_path_definitions(),
        app_key::definition()
        // rules::rev_path_definitions()
    ]

    init: || {
        Ok(())
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {{
         if let EntryValidationData::Create{entry, ..} = validation_data {
             let agent = entry as AgentId;
             if agent.nick == "reject_agent::app" {
                 Err("This agent will always be rejected".into())
             } else {
                 Ok(())
             }
         } else {
             Err("Cannot update or delete an agent at this time".into())
         }
     }}


     receive: |from, msg_json| {
         utils::handle_receive(from, JsonString::from_json(&msg_json))
     }

    functions: [
    // DPKI Trait functions
        init_dpki: {
            inputs: | params: String |,
            outputs: |result: ZomeApiResult<HashString>|,
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
            inputs: | revocation_key: HashString, signed_old_revocation_key:Signature |,
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
            handler: authorizor::handlers::handle_set_authorizor
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
        authorize_device: {
            inputs: | new_agent_hash: HashString, new_agent_signed_xor: Signature |,
            outputs: |result: ZomeApiResult<()>|,
            handler: device_authorization::handlers::handle_authorize_device
        }
        send_handshake_notify: {
            inputs: | to: Address |,
            outputs: |result: ZomeApiResult<Signature>|,
            handler: utils::handle_send_handshake_notify
        }

        // Commit Private AppKey Entry
        register_key: {
            inputs: | app_dna_hash: String, app_name: String, public_key: HashString |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: app_key::handlers::handle_register_key
        }

        // Get all the AppKey Entry
        // UIs should call this function when they want to know all the hApps that are registered in the DNA
        get_registered_key: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<app_key::AppKey>>|,
            handler: app_key::handlers::handle_get_registered_key
        }

    ]

    traits: {
        hc_public [
        init_dpki,
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
        key_status,
        authorize_device,
        send_handshake_notify,
        register_key,
        get_registered_key
        ]
    }
}
