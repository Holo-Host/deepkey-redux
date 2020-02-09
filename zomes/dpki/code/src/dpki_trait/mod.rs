use crate::authorizor::handlers::handle_set_authorizor;
use crate::key_registration::{handlers::handle_create_key_registration, AppKeyType};
use crate::keyset_root::handlers::{handle_get_my_keyset_root, handle_set_keyset_root};
use crate::rules::handlers::create_new_rules;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::signature::Signature,
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::hash::HashString,
    holochain_wasm_utils::api_serialization::sign::SignOneTimeResult,
    AGENT_ADDRESS,
};
use std::time::Duration;

const INITIAL_AUTH_DERIVATION_INDEX: u64 = 1;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitParams {
    revocation_key: Option<HashString>,
    first_deepkey_agent: Option<HashString>,
    signed_auth_key: Signature,
}

#[derive(Serialize, Deserialize, DefaultJson, Debug, Clone)]
struct SetAuthParams {
    authorization_key_path: u64,
    signed_auth_key: Signature,
}

pub fn init(params: String) -> ZomeApiResult<HashString> {
    hdk::debug(format!("AGENT KEY >>>>>>>  {:}", AGENT_ADDRESS.clone()).to_string())?;
    // Checking is initialized
    if !is_initialized()? {
        // DANGER :: Used unrap
        let init_params: InitParams = serde_json::from_str(&params).unwrap();
        match init_params.revocation_key {
            Some(revocation_key) => {
                // If this is the First DeepKey instance for an agent
                // We have to do the following steps
                // - use the sign_one_time() to sign the FirstDeepKeyAgent and revocation Key
                // - set the KeysetRoot
                let sotr: SignOneTimeResult = hdk::sign_one_time(vec![
                    AGENT_ADDRESS.to_string(),
                    revocation_key.to_string(),
                ])?;
                let keyset_root = handle_set_keyset_root(
                    HashString::from(sotr.pub_key),
                    sotr.signatures[0].to_owned(),
                )?;
                hdk::debug(
                    format!("Initial KeysetRoot set:  {:}", keyset_root.clone()).to_string(),
                )?;
                // - set the Rules
                let rules =
                    create_new_rules(&keyset_root, &revocation_key, sotr.signatures[1].to_owned())?;
                hdk::debug(format!("Initial Rules set:  {:}", rules.clone()).to_string())?;
            }
            None => match init_params.first_deepkey_agent {
                Some(_) => {
                    hdk::debug(format!("*******ToDo for FDA*************"))?;
                }
                None => return Err(ZomeApiError::from("Error".to_string())),
            },
        }

        // TODO: Find a beter solution insted of depending on a sleep
        // The sleep is because we need to wait for the rules and KeysetRoot to be registed in the DHT
        hdk::sleep(Duration::from_millis(100))?;
        match handle_set_authorizor(INITIAL_AUTH_DERIVATION_INDEX, init_params.signed_auth_key) {
            Ok(a) => Ok(a),
            Err(e) => Err(e),
        }
    //TODO: Register this DeepKey Agent
    } else {
        Err(ZomeApiError::Internal(
            "INIT ERROR: Already Initialized".to_string(),
        ))
    }
}

pub fn is_initialized() -> ZomeApiResult<bool> {
    match handle_get_my_keyset_root() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn create_agent_keys(context: String) -> ZomeApiResult<()> {
    handle_create_key_registration(1, AppKeyType::AppSig, context.to_owned()).and(
        handle_create_key_registration(1, AppKeyType::AppEnc, context.to_owned()),
    )?;
    Ok(())
}
