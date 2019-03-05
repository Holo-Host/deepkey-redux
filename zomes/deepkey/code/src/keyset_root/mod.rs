use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    signature::Signature
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct KeysetRoot {
    pub first_deepkey_agent: HashString,
    pub root_pubkey: HashString,
    pub fda_signed_by_rootkey: Signature,
}

pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "keyset_root",
        description: "Root hash that would be used as an anchor",
        sharing: Sharing::Public,
        native_type: KeysetRoot,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_kh: KeysetRoot, _validation_data: hdk::ValidationData| {
            {
                // Validating if the source of the keyset_root is the same as the first_deepkey_agent
                let source = &_validation_data.package.chain_header.provenances()[0].0;
                if &_kh.first_deepkey_agent == source {
                    hdk::debug("Succesfully Validated that Source == first_deepkey_agent")?;
                    Ok(())
                }
                else{
                    Err("Could not Validate KeysetRoot: Source is not equal to the first_deepkey_agent".to_string())
                }
            }
        },

        links: [
            to!(
                "%agent_id",
                tag: "deepkey_agent_link_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "rules",
                tag: "rules_link_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
