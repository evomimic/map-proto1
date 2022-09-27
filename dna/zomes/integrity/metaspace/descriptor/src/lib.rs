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
    version: String,
}


pub struct TypeDescriptor {
    pub uid: u8, // ?TODO: encode base64
    pub name: String,
    pub description: Description,
    pub semantic: StringFormat, // IRI
    pub version: SemanticVersion,
    pub created_at: DateTime,
    pub is_implemented: bool, // false means MAP defines but doesn't yet support this type
    pub descriptor: Descriptor,
}

pub enum Descriptor {
    Holon(HolonDescriptor),
    Relationship(RelationshipDescriptor),
    DependentType(DependentTypeDescriptor),
}

pub struct HolonDescriptor {
    pub properties: PropertyDescriptorMap,
    pub identifying_properties: PropertyDescriptorMap,
}

pub struct RelationshipDescriptor {
    id: String,
    created_at: DateTime,
    from_role: RelationshipRole,
    to_role: RelationshipRole,
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
    Prop, // propagate deletion of Holon to related Holons

}
pub struct UnitInterval {
    value: f32, // value can range from 0 to 1, inclusive
}
struct FuzzyBoolean {
    value: UnitInterval, // zero = false, one = true
}

pub enum DependentTypeDescriptor {
    BooleanDescriptor(BooleanDescriptor),
    CollectionDescriptor(CollectionDescriptor),
    CompositeDescriptor,
    EnumDescriptor,
    IntegerDescriptor(IntegerDescriptor<>),
    StringDescriptor(StringDescriptor),
}

pub struct BooleanDescriptor {
    is_fuzzy: bool // if true, this property has FuzzyBoolean value, otherwise just true or false
}

pub struct CollectionDescriptor {
    contains : TypeDescriptor,
    min_items: u32,
    max_items: u32,
    unique_items: bool, // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}

pub struct IntegerDescriptor<T> {
    format: IntegerFormat,
    min_value: T,
    max_value: T,
}

pub enum IntegerFormat {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
}
pub struct IntegerDescriptor {
    format: IntegerFormat,
    min_value: u8,
    max_value: u8,
}


pub struct StringDescriptor {
    min_length: u8,
    max_length: u8,
    pattern: String,
    format: StringFormats,
}

pub enum StringFormats { // are these needed, or should, e.g., Email just be a Composite Type)
    Email,
    IdnEmail, // Internationalized Domain Name email containing non-ASCII script - e.g., Arabic, Chinese, or Cyrillic.
    Hostname,
    IdnHostname,
}

// EXAMPLE COMPOSITE TYPES
pub struct FloatDescriptor {
    significand: IntegerProperty,
    base: IntegerProperty,
    exponent: IntegerProperty,
    precision: IntegerProperty,
}

pub struct DateTimeDescriptor {
    date: DateDescriptor,
    time: TimeDescriptor,
    timezone: TimeZone,
}

pub struct DateDescriptor {
    date: IntegerProperty,
    month: IntegerProperty,
    year: IntegerProperty,
}

pub struct TimeDescriptor {
    seconds: FloatDescriptor,
    hours: IntegerDescriptor,
    
}
