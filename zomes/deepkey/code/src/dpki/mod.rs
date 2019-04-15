use crate::keyset_root::{
    handlers::{
        handle_set_keyset_root,
    }
};
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
    // If this is the First DeepKey instance for an agent
    // We have to do the following steps
    // - use the sign_one_time() to sign the FirstDeepKeyAgent and revocation Key
    // - set the KeysetRoot
    // - set the Rules
    {
        let sotr:SignOneTimeResult = hdk::sign_one_time(vec![AGENT_ADDRESS.to_string()])?;
        handle_set_keyset_root(HashString::from(sotr.pub_key),sotr.signatures[0].to_owned())
    }

    // if this is not the First DeepKey Agent
    // ???
}
