// use hdi::prelude::hash_blake2b;
use hdi::prelude::Timestamp;
// use hdk::prelude::holo_hash::*;
use hc_zome_meta_space_descriptor::{ Descriptor, HolonDescriptor };
struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
    version: String,
}

pub struct Holon<T: Descriptor> {
    uid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion,
    descriptor: HolonDescriptor<T>,
    identifying_properties: T,
    // actions:
    // relationships:

}

impl Holon<T> {
    fn new(namespace_id: u8, local_id: u8, version: SemanticVersion, identifying_properties: T, properties: T) -> Self {        
        // let holon_id = hash(&namespace_id + &local_id);
        // let stamp = time.timestamp();
        Self {
            uid: holon_id,
            namespace_id,
            local_id,
            created_at: stamp,
            version,
            descriptor: HolonDescriptor::new(),
            identifying_properties,
        }
    }
    //fn link()

    
}
