use hdk::prelude::*;
use hc_zome_integrity_type_desc::descriptor::HolonDescriptor;
use hc_zome_coordinator_type_desc_api::queries::DescriptorQueries;
use hc_zome_coordinator_type_desc_api::builder::*;
use hc_zome_coordinator_type_desc_impl::queries_impl::QueryController;


#[hdk_extern]
pub fn create_holon_descriptor_builder(_:()) -> ExternResult<Box<dyn HolonDescriptorBuilder>> {
    let factory = DescriptorBuilderStubsFactory::new();
    factory.make_holon_descriptor()
}

#[hdk_extern]
pub fn get_all_holontypes(_:()) -> ExternResult<Vec<HolonDescriptor>> {
    QueryController::get_all_holontypes()
}