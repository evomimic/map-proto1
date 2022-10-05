use std::collections::BTreeMap;
use std::collections::DateTime;
use hc_zome_integrity_holonspace_holon::SemanticVersion;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Uid {
    _id : u64,
}

pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

/*
    TypeDescriptor is abstract definition representing any kind of TypeDescriptor
*/
pub enum TypeDescriptor {
    Holon(HolonDescriptor),
    Collection(CollectionDescriptor),
    Composite(CompositeDescriptor),
    Relationship(RelationshipDescriptor),
    Boolean(BooleanDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor), // TODO: check if enum variant names conflict with keywords/std types
    Enum(EnumDescriptor),
}

pub struct TypeHeader { // the shared attributes common to all Type Descriptors
    uid: Uid, // factor this out into a separate Identifier type?
    type_name: String,
    description: String,
    semantic_type: Option<String>, // IRI? Enum?
    version: SemanticVersion,
    previous: Option<TypeDescriptor>, // the previous version of this descriptor (assumes linear versioning), Link? Vec<Option> for all versions?
    created_at: DateTime,
    is_dependent: bool, // if true, cannot existing independent of parent object
    is_implemented: bool, // false means MAP defines but doesn't yet support this type
}

pub struct HolonDescriptor {
    header: TypeHeader,
    identifying_properties: CompositeDescriptor,
    properties: CompositeDescriptor,
    // add actions and relationships
}

pub struct RelationshipDescriptor {
    header: TypeHeader,
    source_role: RelationshipRole,
    target_role: RelationshipRole,
}

pub struct RelationshipRole {
    role_name: String,
    holon_type: HolonDescriptor,
    binding_rule: RelationshipBindingRule,
    max_multiplicity: u32,
    min_multiplicity: u32,
    deletion_semantic: DeletionSemantic,
    attraction: UnitInterval,

}
pub enum RelationshipBindingRule {
    Auto, // automatically bind to new version of related holon type
    Manual, // manually decide when to bind to new version of related holon type
}

pub enum DeletionSemantic {
    Block, // prevent deletion of Holon if any Holons are related
    Propagate, // propagate deletion of Holon to related Holons
    Allow, // do nothing with the related Holon

}
pub struct UnitInterval {
    value: f32, // value can range from 0 to 1, inclusive
}
struct FuzzyBoolean {
    value: UnitInterval, // zero = false, one = true
}

pub struct BooleanDescriptor {
    header: TypeHeader,
    is_fuzzy: bool // if true, this property has FuzzyBoolean value, otherwise just true or false
}

pub struct CollectionDescriptor {
    header: TypeHeader,
    contains_items_of_type : TypeDescriptor,
    min_items: u32,
    max_items: u32,
    unique_items: bool, // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}

pub struct CompositeDescriptor {
    header: TypeHeader,
    properties: BtreeMap<String, DependentTypeDescriptor>,
}

/*
    The following enum specifies the subset TypeDescriptors whose instances cannot exist independent
    of a parent instance.

    Dependent types don't have unique identifiers
 */

pub enum DependentTypeDescriptor {
    Composite(CompositeDescriptor),
    Collection(CollectionDescriptor), // but only for collections of Dependent Types
    Boolean(BooleanDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor),
    Enum(EnumDescriptor),
}

pub struct IntegerDescriptor {
    header: TypeHeader,
    format: IntegerFormat,
}

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

pub struct IntegerBaseDescriptor<T> {
    min_value: T,
    max_value: T,
}

pub struct StringDescriptor {
    header: TypeHeader,
    min_length: u32,
    max_length: u32,
    pattern: String,
    format: StringFormat,
}

pub enum StringFormat { // are these needed, or should, e.g., Email just be a Composite Type)
    Email,
    IdnEmail, // Internationalized Domain Name email containing non-ASCII script - e.g., Arabic, Chinese, or Cyrillic.
    Hostname,
    IdnHostname,
}


// EXAMPLE COMPOSITE TYPES
pub struct FloatDescriptor {
    significand: IntegerDescriptor,
    base: IntegerDescriptor,
    exponent: IntegerDescriptor,
    precision: IntegerDescriptor,
}

pub struct DateTimeDescriptor {
    date: DateDescriptor,
    time: TimeDescriptor,
    timezone: TimeZone,
}

pub struct DateDescriptor {
    date: IntegerDescriptor,
    month: IntegerDescriptor,
    year: IntegerDescriptor,
}

pub struct TimeDescriptor {
    seconds: FloatDescriptor,
    hours: IntegerDescriptor,
    
}
