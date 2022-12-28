//use derive_new::new;
// TODO: add use path for descriptor integrity def
/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaspaceBuilder {
    Holon(HolonDescriptorBuilder),
    Collection(Box<CollectionDescriptorBuilder>),
    Composite(CompositeDescriptorBuilder),
    Relationship(RelationshipDescriptorBuilder),
    Boolean(BooleanDescriptorBuilder),
    Integer(IntegerDescriptorBuilder),
    String(StringDescriptorBuilder), // TODO: check if enum variant names conflict with keywords/std types
    // Enum(EnumDescriptor),
}

// NOTE: this is just a partial definition of the DescriptorBuilder Trait that
// focuses only on 'setter' functions.
// ALSO NOTE: setters are provided ONLY for fields that are modifiable by users
// In the case of TypeDescriptors, users are not allowed to set semantic_version, created_at, etc.)
pub trait DescriptorBuilder {
    fn with_type_name(&self, type_name:String)->Self;
    fn with_description(&self, description:String)->Self;
    fn with_is_dependent(&self, is_dependent:bool)->Self;
}

pub trait HolonDescriptorBuilder: DescriptorBuilder {
    fn create()->Box<&mut dyn HolonDescriptorBuilder>;
    fn add_property(&self)-> Self;
}



create(...) // input parameters elided, assume this create returns a Box<dyn DescriptorBuilder>
.with_type_name("My New Holon Type")
.with_description("This is a super cool type")
.with_is_dependent(false);


// enum State {
//     New,
//     Derived,
//     Built,
//     Cancelled,
// }

// enum DescriptorChanges {
//     Unchanged,
//     Breaking,
//     Warning,
//     NonBreaking,
// }


// pub struct BuilderMetaData {
//     state: State,
//     predecessor: TypeDescriptor,
//     observed_changes: DescriptorChanges,

// }


#[derive(new, Default, Debug, Clone, Serialize, Deserialize)]
pub struct HolonDescriptorBuilderImpl {
    header: Box<TypeHeader>,
    // identifying_properties: Box<CompositeDescriptorBuilder>,
    // properties: Box<CompositeDescriptorBuilder>,
    // add actions and relationships
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDescriptorBuilder {
    header: Box<TypeHeaderBuilder>,
    source_role: RelationshipRoleBuilder,
    target_role: RelationshipRoleBuilder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRoleBuilder {
    role_name: String,
    holon_type: HolonDescriptorBuilder,
    binding_rule: RelationshipBindingRule,
    max_multiplicity: u32,
    min_multiplicity: u32,
    deletion_semantic: DeletionSemantic,
    attraction: UnitInterval,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanDescriptorBuilder {
    header: TypeHeaderBuilder,
    is_fuzzy: bool // if true, this property has FuzzyBoolean value, otherwise just true or false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionDescriptorBuilder {
    header: TypeHeader,
    contains_items_of_type: Box<TypeDescriptorBuilder>,
    min_items: u32,
    max_items: u32,
    unique_items: bool, // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeDescriptorBuilder {
    header: TypeHeaderBuilder,
    properties: BTreeMap<String, DependentTypeDescriptorBuilder>,
}

/*
    The following enum specifies the subset TypeDescriptors whose instances cannot exist independent
    of a parent instance.
    Dependent types don't have unique identifiers
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependentTypeDescriptorBuilder {
    Composite(CompositeDescriptorBuilder),
    Collection(CollectionDescriptorBuilder), // but only for collections of Dependent Types
    Boolean(BooleanDescriptorBuilder),
    Integer(IntegerDescriptorBuilder),
    String(StringDescriptorBuilder),
    // Enum(EnumDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerDescriptorBuilder {
    header: TypeHeader,
    format: IntegerFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegerFormat {
    I8(IntegerBaseDescriptorBuilder<i8>),
    I16(IntegerBaseDescriptorBuilder<i16>),
    I32(IntegerBaseDescriptorBuilder<i32>),
    I64(IntegerBaseDescriptorBuilder<i64>),
    I128(IntegerBaseDescriptorBuilder<i128>),
    U8(IntegerBaseDescriptorBuilder<u8>),
    U16(IntegerBaseDescriptorBuilder<u16>),
    U32(IntegerBaseDescriptorBuilder<u32>),
    U64(IntegerBaseDescriptorBuilder<u64>),
    U128(IntegerBaseDescriptorBuilder<u128>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerBaseDescriptorBuilder<T> {
    min_value: T,
    max_value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringDescriptorBuilder {
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

 */