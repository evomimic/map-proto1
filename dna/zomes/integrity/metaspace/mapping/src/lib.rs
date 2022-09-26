use std::collections::BTreeMap;

use hc_zome_integrity_metaspace_descriptor::CompositeDescriptor;

pub struct PropertyMap<K,V> {
    pub identifying_properties: BTreeMap<K,V>,
}

pub struct PropertyDescriptorMap {
    pub properties: CompositeDescriptor,
}

pub struct HolonRelationshipMap {
    pub name: String,
    pub type_: HolonRelationship
}

pub struct ActionMap {
    pub name: String,
    pub type_: HolonAction
}

pub struct HolonRelationship {

}

pub struct HolonAction {

}