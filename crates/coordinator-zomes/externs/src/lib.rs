
use hc_zome_coordinator_type_desc_impl::{DescriptorQueries, QueryController};




#[hdk_extern]
pub fn get_all_holontypes() -> ExternResult<Vec<HolonDescriptor>> {
    QueryController::get_all_holontypes()
}