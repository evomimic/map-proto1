use hdk::prelude::*;

use hc_zome_integrity_type_desc::descriptor::HolonDescriptor;
use hc_zome_coordinator_type_desc_api::queries::DescriptorQueries;
use hc_zome_coordinator_type_desc_impl::queries_impl::DescriptorQueryControllerStubs;

/*
    NOTE: Currently a DescriptorQueryController is being instantiated on every query call
    TODO: Consider creating a service object singleton
 */


#[hdk_extern]
pub fn get_all_holontypes(_:()) -> ExternResult<Vec<HolonDescriptor>> {
    let controller = get_controller();
    controller.get_all_holontypes()
}

// the following helper function is a factory method for selecting DescriptorQueryController
//
fn get_controller()-> Box<dyn DescriptorQueries> {
    Box::new(DescriptorQueryControllerStubs)
}
