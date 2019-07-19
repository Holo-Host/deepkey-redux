use hdk::{
    error::ZomeApiResult,
    holochain_core_types::signature::Signature,
    holochain_wasm_utils::api_serialization::{
        keystore::KeyType,
    },
};

// This function is just for testing
// TODO : DELEATE
pub fn handle_sign_message() -> ZomeApiResult<Signature> {
    // Create Revocation key
        let rev_key = hdk::keystore_derive_key("root_seed".to_string(),  "rev_key".to_string(), KeyType::Signing)?;
        hdk::debug(format!("Revocation Key 1 : {:}",rev_key).to_string())?;
        hdk::keystore_sign("rev_key".to_string(), "HcKciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()).map(Signature::from)
}
