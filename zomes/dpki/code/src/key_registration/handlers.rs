use crate::authorizor;
use crate::key_anchor::KeyAnchor;
use crate::key_registration::{AppKeyType, KeyMeta, KeyRegistration};
use crate::rules;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        chain_header::ChainHeader,
        entry::Entry,
        error::HolochainError,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},
    holochain_wasm_utils::api_serialization::{
        keystore::KeyType,
        query::{QueryArgsOptions, QueryResult},
    },
};
use std::convert::TryFrom;

fn choose_key_type(key_type: &AppKeyType) -> KeyType {
    match key_type {
        AppKeyType::AppUI => return KeyType::Signing,
        AppKeyType::AppSig => return KeyType::Signing,
        AppKeyType::AppEnc => return KeyType::Encrypting,
    }
}

pub fn handle_create_key_registration(
    derivation_index: u64,
    key_type: AppKeyType,
    context: String,
) -> ZomeApiResult<Address> {
    // Validate the key and sign the key wit the auth key
    let derived_key = derive_key(derivation_index, &context, choose_key_type(&key_type))?
        .trim_matches('"')
        .to_owned();
    let derived_key_hashstring = HashString::from(derived_key.to_owned());

    //Get the Auth Kye ID
    let auth_key_signing_keys = sign_key_by_authorization_key(derived_key)?;

    // Registering the Key
    let key_registration = KeyRegistration {
        new_agent_key: derived_key_hashstring.to_owned(),
        authorization_sig: auth_key_signing_keys,
        prior_key: None,      // (missing on Create, required on Update)
        revocation_sig: None, // (missing on Create, required on Update or Delete)
    };
    let key_registration_entry = Entry::App("key_registration".into(), key_registration.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: derived_key_hashstring.to_owned(),
        }
        .into(),
    );
    let key_meta = Entry::App(
        "key_meta".into(),
        KeyMeta {
            new_key: derived_key_hashstring.to_owned(),
            derivation_index: derivation_index,
            key_type: key_type,
            context: context, // some_app_DNA_hash
        }
        .into(),
    );
    // Hopfully we bundle this two commits once we have that feature
    match hdk::commit_entry(&key_registration_entry) {
        Ok(address) => {
            hdk::commit_entry(&key_meta)?;
            hdk::commit_entry(&key_anchor)?;
            Ok(address)
        }
        Err(e) => Err(e),
    }
}

pub fn update_key(
    old_key: HashString,
    signed_old_key: Signature,
    context: String,
) -> ZomeApiResult<()> {
    // Get the Old Meta for details
    let new_key_type;
    let new_derivation_index;
    let old_key_meta_address = get_address_of_key_meta(
        &old_key,
        query_local_chain_for_entry_type("key_meta".to_string())?,
    )?;
    let meta = hdk::get_entry(&old_key_meta_address)?;
    if let Some(Entry::App(_, json_string)) = meta {
        let meta_data = KeyMeta::try_from(json_string)?;
        new_key_type = meta_data.key_type.to_owned();
        new_derivation_index = &meta_data.derivation_index.to_owned() + 1;
    } else {
        return Err(ZomeApiError::Internal(
            "ERROR:(NEED TO BE SOLVED) Not Able to Find meta. i.e. it is possible you are trying to revoke a key from a diffrent device".to_string(),
        ));
    }

    // Need to Derive the new key
    let new_key = derive_key(
        new_derivation_index.to_owned(),
        &context.to_owned(),
        choose_key_type(&new_key_type),
    )?
    .trim_matches('"')
    .to_owned();

    // Finally Update your key
    handle_update_key_registration(
        old_key,
        signed_old_key,
        HashString::from(new_key),
        new_derivation_index,
        new_key_type.to_owned(),
        context,
    )?;

    Ok(())
}

// Update a registered Key
pub fn handle_update_key_registration(
    old_key: HashString,
    signed_old_key: Signature,
    new_key: HashString,
    derivation_index: u64,
    key_type: AppKeyType,
    context: String,
) -> ZomeApiResult<Address> {
    // Fetch the revocation Key
    let revocation_authority = rules::handlers::handle_get_my_rule_details()?;

    // Verify if the right Revocation Key is used to sign the key
    if !hdk::verify_signature(
        Provenance::new(
            revocation_authority[0].entry.revocation_key.to_owned(),
            signed_old_key.to_owned(),
        ),
        String::from(old_key.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature Not Able to be Verified".to_string(),
        ));
    }

    // get Address of the registered old_key
    let old_key_address = get_address_of_key(
        &old_key,
        query_local_chain_for_entry_type("key_registration".to_string())?,
    )?;

    //Get the Auth Kye ID
    let auth_key_signing_keys = sign_key_by_authorization_key(new_key.to_string())?;

    // Registering the Key
    let key_registration = KeyRegistration {
        new_agent_key: new_key.to_owned(),
        authorization_sig: auth_key_signing_keys,
        prior_key: Some(old_key.to_owned()), // (missing on Create, required on Update)
        revocation_sig: Some(signed_old_key), // (missing on Create, required on Update or Delete)
    };
    let key_registration_entry = Entry::App("key_registration".into(), key_registration.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: new_key.to_owned(),
        }
        .into(),
    );
    let old_key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: old_key.to_owned(),
        }
        .into(),
    );
    let old_key_anchor_address = hdk::entry_address(&old_key_anchor)?;

    let key_meta = Entry::App(
        "key_meta".into(),
        KeyMeta {
            new_key: new_key.to_owned(),
            derivation_index: derivation_index,
            key_type: key_type,
            context: context, // some_app_DNA_hash
        }
        .into(),
    );
    // get Address of the registered old_key
    let old_key_meta_address = get_address_of_key_meta(
        &old_key,
        query_local_chain_for_entry_type("key_meta".to_string())?,
    )?;

    match hdk::update_entry(key_registration_entry, &old_key_address) {
        Ok(address) => match hdk::update_entry(key_meta, &old_key_meta_address) {
            Ok(_) => {
                hdk::update_entry(key_anchor, &old_key_anchor_address)?;
                return Ok(address);
            }
            Err(_) => Err(ZomeApiError::from(
                "handle_update_key_registration: Unable to Update Key Meta".to_string(),
            )),
        },
        Err(_) => Err(ZomeApiError::from(
            "handle_update_key_registration: Unable to Update Key".to_string(),
        )),
    }
}

pub fn handle_delete_key_registration(
    old_key: HashString,
    signed_old_key: Signature,
) -> ZomeApiResult<()> {
    // Fetch the revocation Key
    let revocation_authority = rules::handlers::handle_get_my_rule_details()?;

    // Verify if the right Revocation Key is used to sign the key
    if !hdk::verify_signature(
        Provenance::new(
            revocation_authority[0].entry.revocation_key.to_owned(),
            signed_old_key.to_owned(),
        ),
        String::from(old_key.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature Not Able to be Verified".to_string(),
        ));
    }

    // get Address of the registered old_key
    let old_key_address = get_address_of_key(
        &old_key,
        query_local_chain_for_entry_type("key_registration".to_string())?,
    )?;

    // Recreate the Anchor
    let old_key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: old_key.to_owned(),
        }
        .into(),
    );
    let old_key_anchor_address = hdk::entry_address(&old_key_anchor)?;

    // get Address of the registered old_key
    let old_key_meta_address = get_address_of_key_meta(
        &old_key,
        query_local_chain_for_entry_type("key_meta".to_string())?,
    )?;

    match hdk::remove_entry(&old_key_address) {
        Ok(_) => {
            hdk::remove_entry(&old_key_anchor_address)?;
            hdk::remove_entry(&old_key_meta_address)?;
            Ok(())
        }
        Err(_) => Err(ZomeApiError::from(
            "handle_delete_key_registration: Unable to Delete Key".to_string(),
        )),
    }
}

// pub fn handle_get_key_registration(address: Address) -> ZomeApiResult<Option<Entry>> {
//     hdk::get_entry(&address)
// }

// ----------------------
// Helper functions
// ----------------------

fn sign_key_by_authorization_key(key: String) -> Result<Signature, ZomeApiError> {
    //Get the Auth Kye ID
    let auth_key_id = authorizor::handlers::handle_get_authorizor_meta()?;
    let auth_key_src_id = ["auth_key:", &auth_key_id.to_string()].concat();
    let auth_key_signing_keys = hdk::keystore_sign(auth_key_src_id, key)?;
    Ok(Signature::from(auth_key_signing_keys))
}

// Gen Seed and Key
fn derive_key(index: u64, context: &String, key_type: KeyType) -> ZomeApiResult<String> {
    let agent_seed = ["agent_seed:", context, ":", &index.to_string()].concat();
    // let app_key = ["app_key:", context, ":", &index.to_string()].concat();

    let agent_key_id_str;
    match key_type {
        KeyType::Signing => {
            agent_key_id_str = [context.to_owned(), ":sign_key".to_string()].concat()
        }
        KeyType::Encrypting => {
            agent_key_id_str = [context.to_owned(), ":enc_key".to_string()].concat()
        }
    }

    // Check if the agent_seed Exists before
    //*******************
    // TODO : if it exist send the agent_key_id_str back not an Err
    //*******************
    let list_of_secreats = hdk::keystore_list().map(|keystore_ids| keystore_ids.ids)?;
    if list_of_secreats.contains(&agent_key_id_str) {
        return Err(ZomeApiError::Internal(
            "Agent key already Exists".to_string(),
        ));
    } else if !list_of_secreats.contains(&agent_seed) {
        hdk::keystore_derive_seed(
            "root_seed".to_string(),
            agent_seed.to_owned(),
            context.to_string(),
            index.to_owned(),
        )?;
    }
    // NOTE: This will throw an error when called on update, because this string already exists
    // This can be solved only if we can update or deleate the previously created values in the keystore
    hdk::keystore_derive_key(agent_seed.to_owned(), agent_key_id_str, key_type)
}

fn get_address_of_key(
    key: &HashString,
    list: Vec<(ChainHeader, Entry)>,
) -> Result<HashString, HolochainError> {
    let mut address: Vec<HashString> = Vec::new();
    for k in list {
        if let Entry::App(_, json_string) = k.1 {
            let root = KeyRegistration::try_from(json_string)?;
            if &root.new_agent_key == key {
                address.push(k.0.entry_address().to_owned());
            }
        }
    }
    if !address.is_empty() {
        Ok(address[0].to_owned())
    } else {
        Err(HolochainError::ErrorGeneric(
            "get_address_of_key: The values your Searching does not exists".to_string(),
        ))
    }
}

fn get_address_of_key_meta(
    key: &HashString,
    list: Vec<(ChainHeader, Entry)>,
) -> Result<HashString, HolochainError> {
    let mut address: Vec<HashString> = Vec::new();
    for k in list {
        if let Entry::App(_, json_string) = k.1 {
            let root = KeyMeta::try_from(json_string)?;
            if &root.new_key == key {
                address.push(k.0.entry_address().to_owned());
            }
        }
    }
    if !address.is_empty() {
        Ok(address[0].to_owned())
    } else {
        Err(HolochainError::ErrorGeneric(
            "get_address_of_key: The values your Searching does not exists".to_string(),
        ))
    }
}

// Example o/p
// [ [ { entry_type: { App: 'key_registration' }, entry_address: 'QmehfCVBoJXDkMJj1y3sT7Rfp3Dca3vfcZrNuR5MALcvhm',
// provenances: [ [ 'HcSCJC7x6OoOQ6rwooaXCYUmsmv3csccsaxW6vSYXK6tm7rruQj6w9fHPBsbzsa', 'xjh6L/3rQhKp/M42mzLqbXbYnsIMt1jxsSkvlP8a3HEHIwM8m1v4UENe3ZWAt5meXkJL+M5sdE300ts0U7jkCQ==' ] ],
// link: 'QmQB5dvev5DcDaRoEh6rTCJyQtjgNp2otgywTEhWdDAaZt', link_same_type: null, link_update_delete: null, timestamp: '2019-05-01T22:24:57+00:00' },
// { App: [ 'key_registration', '{"newAgentKey":"HcSCJw6d7h53IAh8twROoUTe8qEiibgfxd3AuB9TwU7UktskXWiSyJ6b8334Umz","authorizationSig":"NwG21frp6/GMX/d+0tpY2RdBaWiuHTaBZ5etKixC7P5wDUoM32ewhGMsoesJm+eBzD5tgep616zdCzpeyBNZAQ==","priorKey":null,"revocationSig":null}' ] } ] ]
fn query_local_chain_for_entry_type(
    entry_type: String,
) -> Result<Vec<(ChainHeader, Entry)>, HolochainError> {
    if let QueryResult::HeadersWithEntries(entries_with_headers) = hdk::query_result(
        vec![entry_type].into(),
        QueryArgsOptions {
            headers: true,
            entries: true,
            ..Default::default()
        },
    )? {
        Ok(entries_with_headers
            .into_iter()
            // .filter(|entry| entry.0.link_update_delete().is_none())
            .collect())
    } else {
        Err(HolochainError::ErrorGeneric(
            "Unexpected hdk::query_result".to_string(),
        ))
    }
}
