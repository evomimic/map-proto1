use hdi::prelude::*;
// use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::{ HolonDescriptor, SemanticVersion };
use hc_zome_integrity_metaspace_mapping::{ PropertyMap, ActionMap, HolonRelationshipMap, PropertyDescriptorMap };
use derive_new::new;

#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
#[derive(Clone, new)]
pub struct Holon {
    uuid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    // created_at: Timestamp,
    version: SemanticVersion,
    // properties: PropertyMap<K,V>,
    descriptor: HolonDescriptor,
    actions: ActionMap,
    relationships: HolonRelationshipMap

}

// impl<T> Holon<T> 
// where T: Description + ?Sized
// impl Holon
// {
//     pub fn build() -> HolonBuilder {
//         HolonBuilder::default()
//     }
// }

// struct HolonInitParams<K,V> {
pub struct HolonInitParams {
    uuid: u8,
    namespace_id: u8,
    local_id: u8,
    version: SemanticVersion,
    // properties: PropertyMap<K,V>,
    descriptor: HolonDescriptor,
    actions: ActionMap,
    relationships: HolonRelationshipMap
}

#[derive(Debug, Clone, new)]
// pub struct HolonBuilder<K,V> {
pub struct HolonBuilder {
    uuid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    // created_at: Timestamp,
    version: SemanticVersion,
    // properties: PropertyMap<K,V>,
    descriptor: HolonDescriptor,
    actions: ActionMap,
    relationships: HolonRelationshipMap
}

// impl<K,V> HolonBuilder<K,V> 
// impl HolonBuilder
// where T: Description
// {

//     pub fn cloned(&self) -> Self {
//         Self
//     }

//     pub fn derived(&self) -> Self {
//         Self
//     }
// }