// put the trait definition for get_all_holon_types() here
use hdk::prelude::*;
use hc_zome_integrity_type_desc::descriptor::HolonDescriptor;


pub trait DescriptorQueries {
    fn get_all_holontypes(&self) -> ExternResult<Vec<HolonDescriptor>>;
    
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


