use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;


pub struct PropertyMap<K,V> {
    pub identifying_properties: BTreeMap<K,V>,
    //
}

pub struct PropertyDescriptorMap<T> {
    pub properties: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolonRelationshipMap {
    pub name: String,
    pub type_: HolonRelationship
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMap {
    pub name: String,
    pub type_: HolonAction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolonRelationship {

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolonAction {

}