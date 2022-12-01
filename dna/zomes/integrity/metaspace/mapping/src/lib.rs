use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use derive_new::new;


#[derive(Debug, Default, Clone)]
pub struct PropertyMap<K,V> {
    pub identifying_properties: BTreeMap<K,V>,
    //
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PropertyDescriptorMap<T> {
    pub properties: Vec<T>,
}

#[derive(Debug, new, Clone, Serialize, Deserialize)]
pub struct HolonRelationshipMap {
    pub name: String,
    // pub type_: HolonRelationship
}

#[derive(Debug, new, Clone, Serialize, Deserialize)]
pub struct ActionMap {
    pub name: String,
    // pub type_: HolonAction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolonRelationship {

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolonAction {

}