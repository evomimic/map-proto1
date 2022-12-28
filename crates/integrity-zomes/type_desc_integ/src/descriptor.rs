
pub mod descriptor {
    use hdi::prelude::EntryHash;
    use hdi::prelude::*;
    use serde::Deserialize;
    use serde::Serialize;
    use std::collections::BTreeMap;
    // TODO: add support for DateTime type via chrono and/or chrono-tz
    //    use self::chrono::prelude::*;

    #[hdk_entry_defs]
    #[unit_enum(UnitEntryTypes)]
    pub enum EntryTypes {
        #[entry_def(name = "HolonDescriptor", visibility = "public")]
        Holon(HolonDescriptor),
        //Collection(CollectionDescriptor),
        //Composite(CompositeDescriptor),
        //Relationship(RelationshipDescriptor),
        #[entry_def(name = "BooleanDescriptor", visibility = "public")]
        Boolean(BooleanDescriptor),
        #[entry_def(name = "IntegerDescriptor", visibility = "public")]
        Integer(IntegerDescriptor),
        #[entry_def(name = "StringDescriptor", visibility = "public")]
        String(StringDescriptor),
        // TODO: check if enum variant names conflict with keywords/std types
        // Enum(EnumDescriptor),
    }
    // pub type Oid = EntryHash;
    pub type Oid = u64;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SemanticVersion {
        major: u8,
        minor: u8,
        patch: u8,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub enum BaseType {
        Holon,
        Collection,
        Composite,
        Relationship,
        Boolean,
        Integer,
        String,
        Enum,
    }

    /*
        TypeDescriptor is abstract definition representing any kind of TypeDescriptor
    */

    #[derive(Serialize, Deserialize, Debug)]
    pub enum TypeDescriptor {
        //Holon(Box<HolonDescriptor>),
        //Collection(CollectionDescriptor),
        //Composite(CompositeDescriptor),
        //Relationship(RelationshipDescriptor),
        Boolean(Box<BooleanDescriptor>),
        //Integer(Box<IntegerDescriptor>),
        String(Box<StringDescriptor>),
        // TODO: check if enum variant names conflict with keywords/std types
       // Enum(EnumDescriptor),
    }


    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct TypeHeader {
        // the shared attributes common to all Type Descriptors
        oid: Oid,
        type_name: String,
        base_type: BaseType,
        description: String,
        semantic_type: Option<String>,
        // IRI? Enum?
        version: SemanticVersion,
        // previous: Option<TypeDescriptor>,
        // the previous version of this descriptor (assumes linear versioning), Link? Vec<Option> for all versions?
        // created_at: DateTime<FixedOffset>, -- this can be derived from create action, needn't be stored here
        is_dependent: bool, // if true, cannot existing independent of parent object
    }

    #[hdk_entry_helper]
    #[serde(rename_all = "camelCase")]
    pub struct HolonDescriptor {
        header: TypeHeader,
        identifying_properties: Vec<EntryHash>,
        properties: BTreeMap<String, DependentTypeDescriptor>,
        // add actions and relationships
    }

    /*
        pub struct RelationshipDescriptor {
            header: Box<TypeHeader>,
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
            Auto,
            // automatically bind to new version of related holon type
            Manual, // manually decide when to bind to new version of related holon type
        }

        pub enum DeletionSemantic {
            Block,
            // prevent deletion of Holon if any Holons are related
            Propagate,
            // propagate deletion of Holon to related Holons
            Allow, // do nothing with the related Holon
        }

        pub struct UnitInterval {
            value: f32, // value can range from 0 to 1, inclusive
        }

        struct FuzzyBoolean {
            value: UnitInterval, // zero = false, one = true
        }
    */
    #[hdk_entry_helper]
    #[derive(Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct BooleanDescriptor {
        header: TypeHeader,
        is_fuzzy: bool, // if true, this property has FuzzyBoolean value, otherwise just true or false
    }
    /*
        pub struct CollectionDescriptor {
            header: Box<TypeHeader>,
            contains_items_of_type: TypeDescriptor,
            min_items: u32,
            max_items: u32,
            unique_items: bool,
            // true means duplicate items are not allowed
            is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
        }

        pub struct CompositeDescriptor {
            header: Box<TypeHeader>,
            properties: BtreeMap<String, DependentTypeDescriptor>,
        }
    */
    /*
       The following enum specifies the subset TypeDescriptors whose instances cannot exist independent
       of a parent instance.

       Dependent types don't have unique identifiers
    */
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum DependentTypeDescriptor {
        //Composite(CompositeDescriptor),
        //Collection(CollectionDescriptor),
        // but only for collections of Dependent Types
        Boolean(BooleanDescriptor),
        Integer(IntegerDescriptor),
        String(StringDescriptor),
        //Enum(EnumDescriptor),
    }

    #[hdk_entry_helper]
    #[derive(Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct IntegerDescriptor {
        header: TypeHeader,
        format: IntegerFormat,
        min_value: u128,
        max_value: u128,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub enum IntegerFormat {
        I8(),
        I16(),
        I32(),
        I64(),
        I128(),
        U8(),
        U16(),
        U32(),
        U64(),
        U128(),
    }

    #[hdk_entry_helper]
    #[derive(Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct StringDescriptor {
        header: TypeHeader,
        min_length: u32,
        max_length: u32,
        //pattern: String,
    }
}
