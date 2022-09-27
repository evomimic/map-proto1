use std::collections::BTreeMap;
// use hdi::prelude::hash_blake2b;
// use hdk::prelude::holo_hash::*;
use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::{ Descriptor, HolonDescriptor, SemanticVersion };
use hc_zome_integrity_metaspace::mapping::{ PropertyMap, ActionMap, RelationshipMap, PropertyDescriptorMap };


pub struct Holon<T: Descriptor> {
    uid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion,
    properties: PropertyMap,
    descriptor: HolonDescriptor<T>,
    actions: ActionMap,
    relationships: RelationshipMap

}