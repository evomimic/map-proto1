use hdi::prelude::*;

pub mod testing;
pub use testing::*;
pub use testing::{TestEntry, TestDescriptor};


#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def()]
    MetaSpaceState(MetaSpaceState),
    #[entry_def()]
    Branch(Branch),
    #[entry_def()]
    TestEntry(TestEntry),
    #[entry_def()]
    TestDescriptor(TestDescriptor),
}

#[hdk_entry_helper]
pub struct MetaSpaceState {
    id: String,
    name: String,
}

#[hdk_entry_helper]
pub struct Branch {
    id: String,
    name: String,
    //meta_data
}