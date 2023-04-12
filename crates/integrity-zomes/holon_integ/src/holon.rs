
use hdi::prelude::*;
use std::collections::BTreeMap;

use descriptors::{ HolonDescriptor, };

#[hdk_link_types]
pub enum LinkTypes {
    HolonUpdates,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "Holon", visibility = "public")]
    Holon(Holon),
}
#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct Holon {
    namespace_id: u8,
    local_id: u8,
    //  created_at: Timestamp,
    //version: SemanticVersion,
    properties: BTreeMap<String, PropertyValue>,
    descriptor: HolonDescriptor,
    //  actions: ActionMap,
    //relationships: RelationshipMap
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PropertyValue {
    //IntegerValue(IntegerValue<T>),
    StringValue(StringValue),
    //EnumValue(EnumValue<T>),
    BooleanValue(BooleanValue),
    //CompositeValue(CompositeValue),
    //CollectionValue(CollectionValue<T>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntegerValue<T> {
    //descriptor: IntegerDescriptor,
    value: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringValue {
    //descriptor: StringDescriptor,
    value: String,
}
/*
    pub struct EnumValue<T> {
        descriptor: EnumDescriptor,
        value: T,
    }
*/

/*
    pub struct CompositeValue {
        descriptor: CompositeDescriptor,
        properties: BtreeMap<String,PropertyValue>
    }
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BooleanValue {
    //descriptor: BooleanDescriptor,
    value: bool,
    fuzzy_value: UnitInterval,
}
/*
    pub struct CollectionValue {
        descriptor: CollectionDescriptor,
        items:Vec<T>,
    }
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitInterval {
    value: f32, // ranges from 0 (FALSE) to 1 (TRUE)
}

pub struct RelationshipMap {
    //TODO
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }


