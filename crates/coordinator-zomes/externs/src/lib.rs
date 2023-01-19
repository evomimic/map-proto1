use hdk::prelude::*;
use hc_zome_integrity_type_desc::descriptor::HolonDescriptor;
use hc_zome_coordinator_type_desc_api::queries::DescriptorQueries;
use hc_zome_coordinator_type_desc_api::builder::*;
use hc_zome_coordinator_type_desc_impl::queries_impl::QueryController;


// #[hdk_extern]
// pub fn create_descriptor_builder(factory: DescriptorBuilderStubsFactory) -> ExternResult<Box<dyn DescriptorBuilder>> {
//     create_descriptor_builder(factory)
// }

#[hdk_extern]
pub fn get_all_holontypes(_:()) -> ExternResult<Vec<HolonDescriptor>> {
    QueryController::get_all_holontypes()
}