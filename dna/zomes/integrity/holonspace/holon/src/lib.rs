use hdi::prelude::*;
use std::collections::BTreeMap;
// use hdi::prelude::hash_blake2b;
// use hdk::prelude::holo_hash::*;
// use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::*;

mod holon;
pub use holon::Holon;


#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
#[entry_def()]
  Holon(Holon),
}

pub enum PropertyValue<T> {
  IntegerValue(IntegerValue),
  StringValue(StringValue),
  // EnumValue(EnumValue<T>),
  BooleanValue(BooleanValue),
  CompositeValue(CompositeValue<T>),
  CollectionValue(CollectionValue<T>),
}

pub struct IntegerBaseValue<T> {
  descriptor: IntegerDescriptor,
  value: T,
}

pub enum IntegerValue {
  I8(IntegerBaseValue<i8>),
  I16(IntegerBaseValue<i16>),
  I32(IntegerBaseValue<i32>),
  I64(IntegerBaseValue<i64>),
  I128(IntegerBaseValue<i128>),
  U8(IntegerBaseValue<u8>),
  U16(IntegerBaseValue<u16>),
  U32(IntegerBaseValue<u32>),
  U64(IntegerBaseValue<u64>),
  U128(IntegerBaseValue<u128>),
}

pub struct StringValue {
  descriptor: StringDescriptor,
  value: String,
}

// pub struct EnumValue<T> {
//   descriptor: EnumDescriptor,
//   value: T,
// }

// pub struct DateTimeValue {
//   descriptor: DateTimeDescriptor,
//   properties: BTreeMap<String, CompositeValue>,
// }

pub struct CompositeValue<T> {
  descriptor: CompositeDescriptor,
  properties: BTreeMap<String, PropertyValue<T>>
}

pub struct BooleanValue {
  descriptor: BooleanDescriptor,
  value: bool,
  fuzzy_value: UnitInterval,
}

pub struct CollectionValue<T> {
  descriptor: CollectionDescriptor,
  items:Vec<T>,
}

pub struct UnitInterval {
  value: f32, // ranges from 0 (FALSE) to 1 (TRUE)
}



pub struct RelationshipMap { //TODO

}

// #[hdk_extern]
// pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
//   Ok(ValidateCallbackResult::Valid)
// }
