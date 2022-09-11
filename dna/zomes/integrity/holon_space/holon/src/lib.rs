// use hdi::prelude::hash_blake2b;
use hdi::prelude::Timestamp;
// use hdk::prelude::holo_hash::*;
use hc_zome_meta_space_descriptor::HolonDescriptor;
enum SemanticVersion {

}

pub struct Holon {
    uid: u8, // TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion,
    descriptor: HolonDescriptor,
}

impl Holon {
    fn new(namespace_id: u8, local_id: u8, version: SemanticVersion, identifying_properties: Box<dyn Descriptor>, properties: Box<dyn Descriptor>) -> Self {        
        // let holon_id = hash(&namespace_id + &local_id);
        // let stamp = time.timestamp();

        Self {
            uid: holon_id,
            namespace_id,
            local_id,
            created_at: stamp,
            version,
            descriptor: HolonDescriptor::new()
        }
    }
    //fn link()

    
}

// impl Descriptor for Holon {
//     fn new(&self) -> Self;
    
// }