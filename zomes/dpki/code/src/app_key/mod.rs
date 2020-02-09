use hdk::prelude::*;
use hdk::{
    holochain_json_api::json::{default_to_json, default_try_from_json},
    holochain_persistence_api::hash::HashString,
};
pub mod handlers;
use std::convert::TryFrom;

// This Entry is a Private Entry
// We will use this Entry only for displaying data to User (UI)
// It contains all the hApps data that are running on the same conductor this DNA is running on.Address
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppKey {
    pub app_dna_hash: String,
    pub app_name: String,
    pub public_key: HashString,
}

impl AppKey {
    pub fn new(app_dna_hash: &str, app_name: &str, public_key: &HashString) -> AppKey {
        AppKey {
            app_dna_hash: app_dna_hash.to_owned(),
            app_name: app_name.to_owned(),
            public_key: public_key.to_owned(),
        }
    }

    pub fn app_dna_hash(&self) -> String {
        self.app_dna_hash.clone()
    }

    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    pub fn public_key(&self) -> HashString {
        self.public_key.clone()
    }
}

impl From<AppKey> for JsonString {
    fn from(my_type: AppKey) -> Self {
        default_to_json(my_type)
    }
}

impl TryFrom<JsonString> for AppKey {
    type Error = JsonError;
    fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
        default_try_from_json(json_string)
    }
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "app_key",
        description: "private entry for apps to register their usage of an agentKey on bridge genesis or key updates",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<AppKey>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_r,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                }
            }
        }
    )
}

#[cfg(test)]
mod tests {

    use crate::app_key::{definition, AppKey};
    use hdk::prelude::*;
    use hdk::{
        holochain_core_types::{
            chain_header::test_chain_header,
            entry::entry_type::{AppEntryType, EntryType},
            validation::{EntryLifecycle, EntryValidationData, ValidationData, ValidationPackage},
        },
        holochain_persistence_api::hash::HashString,
    };

    #[test]
    fn app_key_smoke_test() {
        let app_dna_hash = "foo";
        let app_name = "bar";
        let public_key = HashString::from("Hsc...");
        let app_key = AppKey::new(&app_dna_hash, &app_name, &public_key);

        assert_eq!(app_dna_hash.to_string(), app_key.app_dna_hash(),);
        assert_eq!(app_name.to_string(), app_key.app_name(),);
    }

    #[test]
    fn app_key_definition_test() {
        let mut app_key_definition = definition();

        let expected_name = EntryType::from("app_key");
        assert_eq!(expected_name, app_key_definition.name.clone());

        let expected_validation_package_definition = hdk::ValidationPackageDefinition::Entry;
        assert_eq!(
            expected_validation_package_definition,
            (app_key_definition.package_creator)(),
        );

        let app_key_ok = AppKey::new("foo", "now", &HashString::from("Hsc..."));
        let entry = Entry::App(AppEntryType::from("app_key"), app_key_ok.into());
        let validation_data = ValidationData {
            package: ValidationPackage::only_header(test_chain_header()),
            lifecycle: EntryLifecycle::Chain,
        };
        assert_eq!(
            (app_key_definition.validator)(EntryValidationData::Create {
                entry,
                validation_data
            }),
            Ok(()),
        );
    }

}
