use crate::authorizor::Authorizor;
use crate::key_anchor::KeyAnchor;
use crate::rules::{self, Rules};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        chain_header::ChainHeader,
        entry::Entry,
        error::HolochainError,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::hash::HashString,
    holochain_wasm_utils::api_serialization::{
        keystore::KeyType,
        query::{QueryArgsOptions, QueryResult},
    },
    utils, AGENT_ADDRESS,
};

fn generate_auth(index: u64) -> ZomeApiResult<String> {
    let auth_seed = ["auth_seed:", &index.to_string()].concat();
    let auth_key = ["auth_key:", &index.to_string()].concat();
    // Check if the authSeed Exists before
    let list_of_secreats = hdk::keystore_list().map(|keystore_ids| keystore_ids.ids)?;
    if list_of_secreats.contains(&auth_seed) {
        return hdk::keystore_get_public_key(auth_key);
    }
    hdk::keystore_derive_seed(
        "root_seed".to_string(),
        auth_seed.to_owned(),
        "authSeed".to_string(),
        index,
    )?;
    hdk::keystore_derive_key(auth_seed.to_owned(), auth_key, KeyType::Signing)
}

pub fn handle_create_authorizor(
    authorization_key_path: u64,
    signed_auth_key: Signature,
) -> ZomeApiResult<HashString> {
    let revocation_authority = rules::handlers::handle_get_my_rule_details()?;
    let authorization_key =
        HashString::from(generate_auth(authorization_key_path)?.trim_matches('"'));

    match handle_get_authorizor() {
        // Case when authorizor key was already set. We need to update it..
        Ok(authorizor_entry) => update_authorizor(
            &authorization_key,
            signed_auth_key,
            &revocation_authority[0].address,
            authorizor_entry,
            authorization_key_path,
        ),
        // to set initial authorizor key
        Err(_) => create_new_authorizor(
            &authorization_key,
            signed_auth_key,
            &revocation_authority[0].address,
            &revocation_authority[0].entry,
            authorization_key_path,
        ),
    }
}

fn create_new_authorizor(
    authorization_key: &HashString,
    auth_signed_by_revocation_key: Signature,
    revocation_address: &HashString,
    revocation_entry: &Rules,
    authorization_key_path: u64,
) -> ZomeApiResult<HashString> {
    // Verify if the right Revocation Key is used to sign the auth key
    if !hdk::verify_signature(
        Provenance::new(
            revocation_entry.revocation_key.to_owned(),
            auth_signed_by_revocation_key.to_owned(),
        ),
        String::from(authorization_key.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature Not Able to be Verified".to_string(),
        ));
    }

    let authorizor = Authorizor {
        authorization_key: authorization_key.to_owned(),
        revocation_authority: revocation_address.to_owned(),
        revocation_sig: auth_signed_by_revocation_key,
    };
    let authorizor_entry = Entry::App("authorizor".into(), authorizor.into());
    // Create KeyAnchor to see whether they are currently LIVE/valid or have been updated/deleted.
    let key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: authorization_key.to_owned(),
        }
        .into(),
    );
    let meta = Entry::App(
        "auth_key_derivation_path".into(),
        authorization_key_path.into(),
    );

    // Hopfully we bundle this two commits once we have that feature
    match hdk::commit_entry(&authorizor_entry) {
        Ok(_) => {
            hdk::commit_entry(&meta)?;
            hdk::commit_entry(&key_anchor)?;
            Ok(authorization_key.to_owned())
        }
        Err(e) => Err(e),
    }
}

fn update_authorizor(
    authorization_key: &HashString,
    auth_signed_by_revocation_key: Signature,
    revocation_address: &HashString,
    old_auth: Authorizor,
    authorization_key_path: u64,
) -> ZomeApiResult<HashString> {
    if !hdk::verify_signature(
        Provenance::new(
            old_auth.authorization_key.to_owned(),
            auth_signed_by_revocation_key.to_owned(),
        ),
        String::from(authorization_key.to_owned()),
    )? {
        return Err(ZomeApiError::Internal(
            "Signature Not Able to be Verified".to_string(),
        ));
    }

    // Sign wit the old_auth.authorization_key
    // let auth_signed_by_revocation_key = hc_utils::sign("primary_keybundle:sign_key".to_string(),String::from(authorization_key.clone()))?;
    let authorizor = Authorizor {
        authorization_key: authorization_key.to_owned(),
        revocation_authority: revocation_address.to_owned(),
        revocation_sig: auth_signed_by_revocation_key,
    };
    let entry = Entry::App("authorizor".into(), authorizor.into());
    let old_authorizor_address =
        check_vec_if_valid_value(query_local_chain_for_entry_type("authorizor".to_string())?)?;
    let new_key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: authorization_key.to_owned(),
        }
        .into(),
    );

    let old_key_anchor = Entry::App(
        "key_anchor".into(),
        KeyAnchor {
            pub_key: old_auth.authorization_key.to_owned(),
        }
        .into(),
    );
    let old_key_anchor_address = hdk::entry_address(&old_key_anchor)?;

    let new_meta = Entry::App(
        "auth_key_derivation_path".into(),
        authorization_key_path.into(),
    );
    let old_meta = Entry::App(
        "auth_key_derivation_path".into(),
        (authorization_key_path - 1).into(),
    );
    let old_meta_address = hdk::entry_address(&old_meta)?;

    match hdk::update_entry(entry, &old_authorizor_address) {
        Ok(_) => match hdk::update_entry(new_meta, &old_meta_address) {
            Ok(_) => {
                hdk::update_entry(new_key_anchor, &old_key_anchor_address)?;
                Ok(authorization_key.to_owned())
            }
            Err(_) => Err(ZomeApiError::from(
                "update_authorizor: Unable to Update Key Meta".to_string(),
            )),
        },
        Err(_) => Err(ZomeApiError::from(
            "update_authorizor: Unable to Update Key".to_string(),
        )),
    }
}

pub fn handle_get_authorizor() -> ZomeApiResult<Authorizor> {
    let authorizor_address =
        check_vec_if_valid_value(query_local_chain_for_entry_type("authorizor".to_string())?)?;
    utils::get_as_type(authorizor_address)
}

pub fn handle_get_authorizor_meta() -> ZomeApiResult<u64> {
    let authorizor_meta_address = check_vec_if_valid_value(query_local_chain_for_entry_type(
        "auth_key_derivation_path".to_string(),
    )?)?;
    utils::get_as_type(authorizor_meta_address)
}

// ----------------------
// Helper functions
// ----------------------

pub fn check_vec_if_valid_value(
    list: Vec<(ChainHeader, Entry)>,
) -> Result<HashString, HolochainError> {
    let mut address: Vec<HashString> = Vec::new();
    for k in list {
        if &AGENT_ADDRESS.to_string() == &k.0.provenances()[0].0.to_string() {
            address.push(k.0.entry_address().to_owned());
        }
    }
    if !address.is_empty() {
        Ok(address[0].to_owned())
    } else {
        Err(HolochainError::ErrorGeneric(
            "check_vec_if_valid_value: The values your Searching does not exists".to_string(),
        ))
    }
}

// Example o/p
// {"entry_type":{"App":"authorizor"},"entry_address":"QmPZ1u6KezJBcup1siw9dUJ6hAgqix2DxjJzNPUb3Mpj1G",
// "provenances":[["liza------------------------------------------------------------------------------AAAOKtP2nI","TODO"]],
// "link":"QmSdoZMyqJFL7bBfsMP6wZYSmVd1kVqpoGrHuyRuxfqG7Y",
// "link_same_type":null,"link_crud":null,"timestamp":"1970-01-01T00:00:00+00:00"}'
pub fn query_local_chain_for_entry_type(
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
        Ok(entries_with_headers)
    } else {
        Err(HolochainError::ErrorGeneric(
            "Unexpected hdk::query_result".to_string(),
        ))
    }
}
