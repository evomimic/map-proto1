use hdk::prelude::*;

use hc_zome_coordinator_type_desc_api::queries::DescriptorQueries;
use hc_zome_integrity_type_desc::descriptor::descriptor::{HolonDescriptor, TypeHeader};


#[derive(Debug,)]
pub struct QueryController;

// put the implementation for get_all_holon_types() for QueryController here

impl DescriptorQueries for QueryController {

    fn get_all_holontypes() -> ExternResult<Vec<HolonDescriptor>> {

        let descriptor1 = HolonDescriptor {
            header: TypeHeader::new("ex1".to_string(), "desc1".to_string())
        };

        let descriptor2 = HolonDescriptor {
            header: TypeHeader::new("ex2".to_string(), "desc2".to_string())
        };

        let descriptor3 = HolonDescriptor {
            header: TypeHeader::new("ex3".to_string(), "desc3".to_string())
        };

        let descriptors_vec = vec![descriptor1, descriptor2, descriptor3];
        
       Ok(descriptors_vec)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(non_snake_case)]
mod tests {

  use super::*; // allows testing of private functions

  #[test]
  fn test_get_all_holon_types() {
    let descriptors = QueryController::get_all_holontypes();
    println!("{:?}", descriptors);
  }

}