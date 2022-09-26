use std::collections::BTreeMap;
// use hdi::prelude::hash_blake2b;
// use hdk::prelude::holo_hash::*;
use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::{ Descriptor, HolonDescriptor };
use hc_zome_integrity_property::PropertyMap;

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
    properties: PropertyMap,
    // actions:
    // relationships:

}

impl Holon<T> {
    fn new(namespace_id: u8, local_id: u8, version: SemanticVersion, properties: PropertyMap) -> Self {        
        // let holon_id = hash(&namespace_id + &local_id);
        // let stamp = time.timestamp();
        Self {
            uid: holon_id,
            namespace_id,
            local_id,
            created_at: stamp,
            version,
            descriptor: HolonDescriptor::new(),
            properties,
        }
    }
    //fn link()

    
}
