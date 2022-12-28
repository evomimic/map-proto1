
use hc_zome_coordinator_type_desc_api::DescriptorQueries;

pub struct QueryController;

// put the implementation for get_all_holon_types() for QueryController here

impl DescriptorQueries for QueryController {

    fn get_all_holontypes() -> ExternResult<Vec<HolonDescriptor>> {

        let descriptor1 = HolonDescriptor {
            header: Box::new(HolonDescriptor::new(TypeHeader::new("ex1".to_string(), "desc1".to_string())))
        };

        let descriptor2 = HolonDescriptor {
            header: Box::new(HolonDescriptor::new(TypeHeader::new("ex2".to_string(), "desc2".to_string())))
        };

        let descriptor3 = HolonDescriptor {
            header: Box::new(HolonDescriptor::new(TypeHeader::new("ex3".to_string(), "desc3".to_string())))
        };

        let descriptors_vec = vec![descriptor1, descriptor2, descriptor3];

       Ok(descriptors_vec)
    }
}