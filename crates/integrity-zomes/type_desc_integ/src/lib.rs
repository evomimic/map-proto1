use hdi::prelude::*;

use descriptors::descriptor::{ HolonDescriptor, };

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "HolonDescriptor", visibility = "public")]
    HolonDescriptor(HolonDescriptor),
    //Collection(CollectionDescriptor),
    //Composite(CompositeDescriptor),
    //Relationship(RelationshipDescriptor),
    // #[entry_def(name = "BooleanDescriptor", visibility = "public")]
    // Boolean(BooleanDescriptor),
    // #[entry_def(name = "IntegerDescriptor", visibility = "public")]
    // Integer(IntegerDescriptor),
    // #[entry_def(name = "StringDescriptor", visibility = "public")]
    // String(StringDescriptor),
    // TODO: check if enum variant names conflict with keywords/std types
    // Enum(EnumDescriptor),
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
