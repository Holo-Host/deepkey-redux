use crate::keyset_root::handlers::handle_set_keyset_root;
use crate::rules::handlers::create_new_rules;
use hdk::{
    error::ZomeApiResult,
    AGENT_ADDRESS,
    holochain_wasm_utils::api_serialization::sign::SignOneTimeResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    hash::HashString,
};

pub fn init () -> ZomeApiResult<Address>{
    // TODO :Generate Revocation Key 
    let revocation_key: HashString = HashString::from("Revocation...............Key".to_string());
    /// If this is the First DeepKey instance for an agent
    /// We have to do the following steps
    /// - use the sign_one_time() to sign the FirstDeepKeyAgent and revocation Key
    /// - set the KeysetRoot
    let sotr:SignOneTimeResult = hdk::sign_one_time(vec![AGENT_ADDRESS.to_string(),revocation_key.to_string()])?;
    let keyset_root = handle_set_keyset_root(HashString::from(sotr.pub_key), sotr.signatures[0].to_owned())?;
    /// -set the Rules
    create_new_rules(&keyset_root, &revocation_key, sotr.signatures[1].to_owned())?;

    //TODO: if this is not the First DeepKey Agent
    // ???

    Ok(keyset_root)
}
