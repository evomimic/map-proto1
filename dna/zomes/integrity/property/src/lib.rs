use std::collections::BTreeMap;

pub struct PropertyMap<K,V> {
    identifying_properties: BTreeMap<K,V>,
}