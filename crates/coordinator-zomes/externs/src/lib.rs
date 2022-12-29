use hdk::prelude::*;
use hc_zome_integrity_type_desc::descriptor::descriptor::HolonDescriptor;
use hc_zome_coordinator_type_desc_api::queries::DescriptorQueries;
use hc_zome_coordinator_type_desc_impl::queries_impl::QueryController;



#[hdk_extern]
pub fn get_all_holontypes(_input: String) -> ExternResult<Vec<HolonDescriptor>> {
    QueryController::get_all_holontypes()
}