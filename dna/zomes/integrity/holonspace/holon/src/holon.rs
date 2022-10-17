use hdi::prelude::*;
use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::{ HolonDescriptor, SemanticVersion };
use hc_zome_integrity_metaspace_mapping::{ PropertyMap, ActionMap, HolonRelationshipMap, PropertyDescriptorMap };


#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Holon {
    uuid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion,
    // properties: PropertyMap<K,V>,
    descriptor: HolonDescriptor,
    actions: ActionMap,
    relationships: HolonRelationshipMap

}

// impl<T> Holon<T> 
// where T: Description + ?Sized
// {
//     pub fn builder() -> HolonBuilder {
//         HolonBuilder::default()
//     }
// }

struct HolonInitParams<K,V> {
    namespace_id: u8,
    local_id: u8,
    version: SemanticVersion,
    properties: PropertyMap<K,V>,
    descriptor: HolonDescriptor,
    actions: ActionMap,
    relationships: HolonRelationshipMap
}

// #[derive(Default)]
// pub struct HolonBuilder<K,V> {
//     uuid: u8, // ?TODO: encode base64
//     namespace_id: u8,
//     local_id: u8,
//     // created_at: Timestamp,
//     version: SemanticVersion,
//     properties: PropertyMap<K,V>,
//     descriptor: HolonDescriptor,
//     actions: ActionMap,
//     relationships: HolonRelationshipMap
// }

// impl<K,V> HolonBuilder<K,V> 
// // where T: Description
// {
//     pub fn new(params: HolonInitParams<K,V>) -> Self {

//         Self {
//             uuid: "example".hash(),
//             namespace_id: params.namespace_id,
//             local_id: params.local_id,
//             // created_at: Timestamp, //? how to do time stamp ?
//             version: params.version,
//             properties:  params.properties,
//             descriptor: params.descriptor,
//             actions: params.actions,
//             relationships: params.relationships,
//         }
//     }

//     pub fn cloned(&self) -> Self {
//         Self
//     }

//     pub fn derived(&self) -> Self {
//         Self
//     }
// }