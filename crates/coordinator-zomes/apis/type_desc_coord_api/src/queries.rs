// put the trait definition for get_all_holon_types() here

use hc_zome_integrity_metaspace_descriptor::*;
use hc_zome_integrity_metaspace_descriptor::HolonDescriptor;


pub trait DescriptorQueries {
    fn get_all_holontypes() -> ExternResult<Vec<HolonDescriptor>>;
    
}


// pub fn create(input: ProcessInput) -> ExternResult<OpResult> {
//     match input {
//         // ProcessInput::Holon(h) => ,
//         ProcessInput::TypeDescriptor(td) => create_typedescriptor(td),
//     }
// }

// fn get_typedescriptor(action_hash: ActionHash) -> ExternResult<Option<Record>> {
//     get(action_hash, GetOptions::default())
// }


// pub fn create_typedescriptor(builder_input: TypeDescriptorBuilderInput) -> ExternResult<impl DescriptorBuilder> {

    
// }



#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(non_snake_case)]
mod tests {

  use super::*; // allows testing of private functions

  #[test]
  fn test_get_all_holon_types() {
    let descriptors = HolonDescriptor::get_all_holontypes();
    println!(descriptors)
  }

}