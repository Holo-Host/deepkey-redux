use crate::keyset_root::handlers::handle_set_keyset_root;
use crate::rules::handlers::create_new_rules;
use hdk::{
    error::ZomeApiResult,
    AGENT_ADDRESS,
    holochain_wasm_utils::api_serialization::sign::SignOneTimeResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitParams {
    revocation_key: String
}

pub fn init (params: String) -> ZomeApiResult<Address>{

    // DANGER :: Used unrap
    let init_params: InitParams = serde_json::from_str(&params).unwrap();
    // Conver the string to get the Json you expect
    // let init_params = InitParams::try_from(params)?;
    let revocation_key = HashString::from(init_params.revocation_key.to_owned());

    // If this is the First DeepKey instance for an agent
    // We have to do the following steps
    // - use the sign_one_time() to sign the FirstDeepKeyAgent and revocation Key
    // - set the KeysetRoot
    let sotr:SignOneTimeResult = hdk::sign_one_time(vec![AGENT_ADDRESS.to_string(),revocation_key.to_string()])?;
    let keyset_root = handle_set_keyset_root(HashString::from(sotr.pub_key), sotr.signatures[0].to_owned())?;
    hdk::debug(format!("Initial KeysetRoot set:  {:}",keyset_root.clone()).to_string())?;
    // - set the Rules
    let rules = create_new_rules(&keyset_root, &revocation_key, sotr.signatures[1].to_owned())?;
    hdk::debug(format!("Initial Rules set:  {:}",rules.clone()).to_string())?;

    //TODO: if this is not the First DeepKey Agent
    // ???

    Ok(keyset_root)
}

// TODO
// NOTE: Just for the testing
pub fn is_initialized () -> ZomeApiResult<bool>{
    Ok(false)
}
