use hdk::{
    error::{ZomeApiResult,ZomeApiError},
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    chain_header::ChainHeader,
    entry::Entry,
    error::HolochainError,
    hash::HashString,
    signature::Signature,
};
use hdk::{
    holochain_wasm_utils::api_serialization::{
            query::{
                QueryArgsOptions, QueryResult,
            },
    }
};
use crate::keyset_root::KeysetRoot;

pub fn handle_set_keyset_root(root_pubkey:HashString,signature:Signature) -> ZomeApiResult<Address>   {
    // Check if the keyset_root Exists
    match handle_get_my_keyset_root(){
        Ok(address) => Ok(address),
        Err(_)=>{
            let root : KeysetRoot = KeysetRoot {
                first_deepkey_agent: HashString::from(AGENT_ADDRESS.to_string()),
                root_pubkey: root_pubkey, // How to get the OTKey?
                fda_signed_by_rootkey: signature, // Need Sign Functions to sign the fda wit the rootkey
            };
            let entry = Entry::App("keyset_root".into(), root.into());
            let entry_addr = hdk::commit_entry(&entry)?;
            hdk::link_entries(&entry_addr,&AGENT_ADDRESS, "deepkey_agent_link_tag")?;
            Ok(entry_addr)
        }
    }
}

// pub fn handle_get_keyset_root_entry(address:Address) -> ZomeApiResult<utils::GetLinksLoadResult<KeysetRoot>> {
//     utils::get_links_and_load_type(&address,"deepkey_agent_link_tag")
// }

pub fn handle_get_my_keyset_root()->ZomeApiResult<HashString>{
    match get_keyset_root_from_source_chain(){
        Ok(keyset_root_list) => {
            let mut address:Vec<HashString>=Vec::new();
            for k in keyset_root_list {
                if &AGENT_ADDRESS.to_string() == &k.0.provenances()[0].0.to_string(){
                    address.push(k.0.entry_address().to_owned());
                }
            }
            if !address.is_empty() {
                Ok(address[0].to_owned())
            }
            else{
                Err(ZomeApiError::from("handle_get_my_keyset_root: No KeysetRoot Exists".to_string()))
            }
        }
        Err(e) =>{
         Err(ZomeApiError::from(e))
        }
    }
}

// Example o/p
// {"entry_type":{"App":"keyset_root"},"entry_address":"QmPZ1u6KezJBcup1siw9dUJ6hAgqix2DxjJzNPUb3Mpj1G",
// "provenances":[["liza------------------------------------------------------------------------------AAAOKtP2nI","TODO"]],
// "link":"QmSdoZMyqJFL7bBfsMP6wZYSmVd1kVqpoGrHuyRuxfqG7Y",
// "link_same_type":null,"link_crud":null,"timestamp":"1970-01-01T00:00:00+00:00"}'
pub fn get_keyset_root_from_source_chain() -> Result<Vec<(ChainHeader,Entry)>,HolochainError> {
    if let QueryResult::HeadersWithEntries( entries_with_headers ) = hdk::query_result(
        vec![
            "keyset_root",
        ].into(),
        QueryArgsOptions{ headers: true, entries: true, ..Default::default()})? {
        Ok(entries_with_headers)
    } else {
        Err(HolochainError::ErrorGeneric( format!("Unexpected hdk::query_result")))
    }
}
