use hdi::prelude::*;
use std::collections::BTreeMap;
// use hdi::prelude::hash_blake2b;
// use hdk::prelude::holo_hash::*;
use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::*;
use hc_zome_integrity_metaspace::mapping::{ PropertyMap, ActionMap, RelationshipMap, PropertyDescriptorMap };

mod holon;
pub use holon::Holon;


#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
#[entry_def()]
  Holon(Holon),

}
pub struct Holon {
  uid: u8, // ?TODO: encode base64
  namespace_id: u8,
  local_id: u8,
  created_at: Timestamp,
  version: SemanticVersion,
  properties: PropertyMap,
  descriptor: HolonDescriptor,
//  actions: ActionMap,
  relationships: RelationshipMap
}

pub struct PropertyMap {
  properties: BtreeMap<String,PropertyValue>
}

pub enum PropertyValue {
  IntegerValue(IntegerValue<T>),
  StringValue(StringValue),
  EnumValue(EnumValue<T>),
  BooleanValue(BooleanValue),
  CompositeValue(CompositeValue),
  CollectionValue(CollectionValue<T>),
}
pub struct IntegerValue<T> {
  descriptor: IntegerDescriptor,
  value:T,
}
pub struct StringValue {
  descriptor: StringDescriptor,
  value: String,
}

pub struct EnumValue<T> {
  descriptor: EnumDescriptor,
  value: T,
}

pub struct CompositeValue {
  descriptor: CompositeDescriptor,
  properties: BtreeMap<String,PropertyValue>
}

pub struct BooleanValue {
  descriptor: BooleanDescriptor,
  value: bool,
  fuzzy_value: UnitInterval,
}

pub struct CollectionValue {
  descriptor: CollectionDescriptor,
  items:Vec<T>,
}

pub struct UnitInterval {
  value: f32, // ranges from 0 (FALSE) to 1 (TRUE)
}



pub struct RelationshipMap { //TODO

}
#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
