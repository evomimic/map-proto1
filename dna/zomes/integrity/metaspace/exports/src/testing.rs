use hdi::prelude::*;


#[hdk_entry_helper]
#[derive(Clone)]
pub struct TestHolonDescriptor {
  test_field: String
}

impl TestHolonDescriptor {
  pub fn new(test_field: String) -> Self {
    Self {
      test_field
    }
  }
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct TestEntry {
  pub example_field: String,
  //another_test_field:
}

#[hdk_entry_helper]
#[derive(Clone)]
pub enum TestDescriptor {
    Holon(TestHolonDescriptor),
    // Collection(Box<CollectionDescriptor>),
    // Composite(CompositeDescriptor),
    // Relationship(RelationshipDescriptor),
    // Boolean(BooleanDescriptor),
    // Integer(IntegerDescriptor),
    // String(StringDescriptor),
}

