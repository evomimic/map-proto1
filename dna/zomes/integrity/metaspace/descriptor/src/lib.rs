use std::collections::BTreeMap;
use derive_new::new;
use hdi::prelude::*;
// use hdi::prelude::Timestamp;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Uuid {
    uuid : u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

//pub struct SemanticVersionTree
/*
    TypeDescriptor is abstract definition representing any kind of TypeDescriptor
*/
#[hdk_entry_helper]
#[derive(Clone)]
pub enum TypeDescriptor {
    Holon(HolonDescriptor),
    // Collection(Box<CollectionDescriptor>),
    // Composite(CompositeDescriptor),
    // Relationship(RelationshipDescriptor),
    // Boolean(BooleanDescriptor),
    // Integer(IntegerDescriptor),
    // String(StringDescriptor), // TODO: check if enum variant names conflict with keywords/std types
    // Enum(EnumDescriptor),
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def()]
    TypeDescriptor(TypeDescriptor),
}

//pub struct DeprecatedTypeDescriptors

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeHeader { // the shared attributes common to all Type Descriptors
    // uuid: Uuid, // factor this out into a separate Identifier type?
    type_name: String,
    description: String,
    // semantic_type: Option<String>, // IRI? Enum?
    // version: SemanticVersion,
    // previous: Box<Option<TypeDescriptor>>, // the previous version of this descriptor (assumes linear versioning), Link? Vec<Option> for all versions?
    // created_at: Timestamp,
    // is_dependent: bool, // if true, cannot existing independent of parent object
    // is_implemented: bool, // false means MAP defines but doesn't yet support this type
}

#[derive(Debug, new, Clone, Serialize, Deserialize)]
pub struct HolonDescriptor {
    header: Box<TypeHeader>,
    // identifying_properties: Box<CompositeDescriptor>,
    // properties: Box<CompositeDescriptor>,
    // add actions and relationships
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDescriptor {
    header: Box<TypeHeader>,
    source_role: RelationshipRole,
    target_role: RelationshipRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRole {
    role_name: String,
    holon_type: HolonDescriptor,
    binding_rule: RelationshipBindingRule,
    max_multiplicity: u32,
    min_multiplicity: u32,
    deletion_semantic: DeletionSemantic,
    attraction: UnitInterval,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipBindingRule {
    Auto, // automatically bind to new version of related holon type
    Manual, // manually decide when to bind to new version of related holon type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletionSemantic {
    Block, // prevent deletion of Holon if any Holons are related
    Propagate, // propagate deletion of Holon to related Holons
    Allow, // do nothing with the related Holon

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitInterval {
    value: f32, // value can range from 0 to 1, inclusive
}
// struct FuzzyBoolean {
//     value: UnitInterval, // zero = false, one = true
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanDescriptor {
    header: TypeHeader,
    is_fuzzy: bool // if true, this property has FuzzyBoolean value, otherwise just true or false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionDescriptor {
    header: TypeHeader,
    contains_items_of_type: Box<TypeDescriptor>,
    min_items: u32,
    max_items: u32,
    unique_items: bool, // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeDescriptor {
    header: TypeHeader,
    properties: BTreeMap<String, DependentTypeDescriptor>,
}

/*
    The following enum specifies the subset TypeDescriptors whose instances cannot exist independent
    of a parent instance.

    Dependent types don't have unique identifiers
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependentTypeDescriptor {
    Composite(CompositeDescriptor),
    Collection(CollectionDescriptor), // but only for collections of Dependent Types
    Boolean(BooleanDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor),
    // Enum(EnumDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerDescriptor {
    header: TypeHeader,
    format: IntegerFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegerFormat {
    I8(IntegerBaseDescriptor<i8>),
    I16(IntegerBaseDescriptor<i16>),
    I32(IntegerBaseDescriptor<i32>),
    I64(IntegerBaseDescriptor<i64>),
    I128(IntegerBaseDescriptor<i128>),
    U8(IntegerBaseDescriptor<u8>),
    U16(IntegerBaseDescriptor<u16>),
    U32(IntegerBaseDescriptor<u32>),
    U64(IntegerBaseDescriptor<u64>),
    U128(IntegerBaseDescriptor<u128>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerBaseDescriptor<T> {
    min_value: T,
    max_value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringDescriptor {
    header: TypeHeader,
    min_length: u32,
    max_length: u32,
    pattern: String,
    format: StringFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringFormat { // are these needed, or should, e.g., Email just be a Composite Type)
    Email,
    IdnEmail, // Internationalized Domain Name email containing non-ASCII script - e.g., Arabic, Chinese, or Cyrillic.
    Hostname,
    IdnHostname,
}

// pub struct EnumDescriptor {

// }


// EXAMPLE COMPOSITE TYPES
// pub struct FloatDescriptor {
//     significand: IntegerDescriptor,
//     base: IntegerDescriptor,
//     exponent: IntegerDescriptor,
//     precision: IntegerDescriptor,
// }

// pub struct DateTimeDescriptor {
//     date: DateDescriptor,
//     time: TimeDescriptor,
//     // timezone: TimeZone,
// }

// pub struct DateDescriptor {
//     date: IntegerDescriptor,
//     month: IntegerDescriptor,
//     year: IntegerDescriptor,
// }

// pub struct TimeDescriptor {
//     seconds: FloatDescriptor,
//     hours: IntegerDescriptor,
    
// }
