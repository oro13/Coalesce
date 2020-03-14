#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};


// see https://developer.holochain.org/api/0.0.45-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Template {
    title: String,
    description: String,
    suggested_min: int32,
    suggested_max: int32,
    roles: Vec<TemplateRole>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct TemplateRole {
    name: String,
    description: String
}

pub fn handle_create_template(entry: Template) -> ZomeApiResult<Address> {
    let entry = Entry::App("template".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_template(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn template_definition() -> ValidatingEntryType {
    entry!(
        name: "template",
        description: "An archetype for organize events of a specific nature",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
}

define_zome! {
    entries: [
       template_definition()
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_template {
            inputs: |entry: Template|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_template
        }
        get_template: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Template>>|,
            handler: handle_get_template
        }
    ]

    traits: {
        hc_public [create_my_entry,get_my_entry]
    }
}
