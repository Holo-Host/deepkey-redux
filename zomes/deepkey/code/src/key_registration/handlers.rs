use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_wasm_utils::api_serialization::keystore::KeyType
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
    signature::Signature,
    hash::HashString
};
use crate::key_registration::{
    KeyRegistration,
    AppKeyType
};
use crate::authorizor;
use crate::key_anchor::KeyAnchor;

fn choose_key_type(key_type: AppKeyType) -> KeyType {
    match key_type {
        AppKeyType::AppUI => return KeyType::Signing,
        AppKeyType::AppSig => return KeyType::Signing,
        AppKeyType::AppEnc => return KeyType::Encrypting
    }
}

pub fn handle_create_key_registration(new_key:HashString, derivation_index: u64, key_type:AppKeyType, context:String) -> ZomeApiResult<Address> {
// Validate the key and sign the key wit the auth key
    let derived_key = derive_key(derivation_index, context, choose_key_type(key_type))?.trim_matches('"').to_owned();
    let derived_key_hashstring = HashString::from(derived_key.to_owned());
    if derived_key_hashstring != new_key {
        return Err(ZomeApiError::Internal("DeepKey Error : The derivation path does not match the key you passed in".to_string()))
    }

//Get the Auth Kye ID
    let auth_key_id = authorizor::handlers::handle_get_authorizor_meta()?;
    let auth_key_src_id = ["auth_key:",&auth_key_id.to_string()].concat();
    let auth_key_signing_keys = hdk::keystore_sign(auth_key_src_id, derived_key)?;

// Registering the Key
    let key_registration = KeyRegistration {
        new_agent_key: new_key.clone(),
        authorization_sig: Signature::from(auth_key_signing_keys),
        prior_key: None, // (missing on Create, required on Update)
        revocation_sig: None, // (missing on Create, required on Update or Delete)
    };
    let key_registration_entry = Entry::App("key_registration".into(), key_registration.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App("key_anchor".into(), KeyAnchor{
        pub_key : new_key
    }.into());

    // Hopfully we bundle this two commits once we have that feature
    match hdk::commit_entry(&key_registration_entry){
        Ok(address) => {
            hdk::commit_entry(&key_anchor)?;
            Ok(address)
        },
        Err(e) => Err(e)
    }
}

// pub fn handle_get_key_registration(address: Address) -> ZomeApiResult<Option<Entry>> {
//     hdk::get_entry(&address)
// }


// **********************
// Gen Seed and Key
// **********************

fn derive_key(index:u64, context: String, key_type: KeyType) -> ZomeApiResult<String> {
    let app_seed = ["app_seed:",&context.to_string(),":",&index.to_string()].concat();
    let app_key = ["app_key:",&context.to_string(),":",&index.to_string()].concat();
    // Check if the appSeed Exists before
    //*******************
    // TODO : if it exist send the app_key back not an Err
    //*******************
    let list_of_secreats = hdk::keystore_list().map(|keystore_ids| keystore_ids.ids)?;
    if list_of_secreats.contains(&app_seed){
        return Err(ZomeApiError::Internal("App key path seed already Exists".to_string()))
    }
    hdk::keystore_derive_seed("root_seed".to_string(), app_seed.to_owned(), context.to_string(), index)?;
    hdk::keystore_derive_key(app_seed.to_owned(),  app_key, key_type)
}
