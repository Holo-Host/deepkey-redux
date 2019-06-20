#![feature(try_from)]
#[macro_use]
extern crate serde_derive;
use core::convert::TryFrom;
use hdk::{
    self,
    holochain_core_types::{
    	hash::HashString,
    	entry::{AppEntryValue, Entry},
    	cas::content::AddressableContent,
        signature::Signature,
        link::LinkMatch
    },
    error::{ZomeApiResult, ZomeApiError}
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadElement<T> {
	pub address: HashString,
	pub entry: T
}

pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;


pub fn get_links_and_load_type<
	R: TryFrom<AppEntryValue>
>(
    base: &HashString,
    link_type: Option<String>
) -> ZomeApiResult<GetLinksLoadResult<R>> {
    let link_type = match link_type {Some(ref s) => LinkMatch::Regex(s.as_ref()), None => LinkMatch::Any};

	let link_load_results = hdk::get_links_and_load(base, link_type, LinkMatch::Exactly(""))?;

	Ok(link_load_results
	.iter()
	.map(|maybe_entry| {

		match maybe_entry {
			Ok(entry) => {
				match entry {
					Entry::App(_, entry_value) => {
						let typed_entry = R::try_from(entry_value.to_owned())
						.map_err(|_| ZomeApiError::Internal(
							"Could not convert get_links result to requested type".to_string())
						)?;

			            Ok(GetLinksLoadElement::<R>{
			                entry: typed_entry,
			                address: entry.to_owned().address()
			            })
					},
					_ => Err(ZomeApiError::Internal(
						"get_links did not return an app entry".to_string())
					)
				}
			},
			_ => Err(ZomeApiError::Internal(
				"get_links did not return an app entry".to_string())
			)
		}
	})
	.filter_map(Result::ok)
	.collect())
}

pub fn sign(key_id: String, message: String) -> ZomeApiResult<Signature> {
    if key_id == "" {
        hdk::sign(message).map(Signature::from)
    } else {
        hdk::keystore_sign(key_id, message).map(Signature::from)
    }
}
