use hdi::prelude::Timestamp;
use hc_zome_integrity_metaspace_descriptor::{ Descriptor, HolonDescriptor, SemanticVersion };
use hc_zome_integrity_metaspace::mapping::{ PropertyMap, ActionMap, RelationshipMap, PropertyDescriptorMap };


#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Holon<T: Descriptor> {
    uuid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion,
    properties: PropertyMap,
    descriptor: HolonDescriptor<T>,
    actions: ActionMap,
    relationships: RelationshipMap

}

impl<T> Holon<T> 
where T: Descriptor + ?Sized
{
    pub fn builder() -> HolonBuilder {
        HolonBuilder::default()
    }
}

struct HolonInitParams<T: Descriptor> {
    namespace_id: u8,
    local_id: u8,
    version: SemanticVersion,
    properties: PropertyMap,
    descriptor: HolonDescriptor<T>,
    actions: ActionMap,
    relationships: RelationshipMap
}

#[derive(Default)]
pub struct HolonBuilder<T: Descriptor> {
    uuid: u8, // ?TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    // created_at: Timestamp,
    version: SemanticVersion,
    properties: PropertyMap,
    descriptor: HolonDescriptor<T>,
    actions: ActionMap,
    relationships: RelationshipMap
}

impl<T> HolonBuilder<T> 
where T: Descriptor
{
    pub fn new(params: HolonInitParams<T>) -> Self {

        Self {
            uuid: "example".hash(),
            namespace_id: params.namespace_id,
            local_id: params.local_id,
            // created_at: Timestamp, //? how to do time stamp ?
            version: params.version,
            properties:  params.properties,
            descriptor: params.descriptor,
            actions: params.actions,
            relationships: params.relationships,
        }
    }

    pub fn cloned(self) -> Holon {

        Holon { 
            uuid: u8, // ?TODO: encode base64
            namespace_id: u8,
            local_id: u8,
            created_at: Timestamp,
            version: SemanticVersion,
            properties: PropertyMap,
            descriptor: HolonDescriptor,
            actions: ActionMap,
            relationships: RelationshipMap
        }
    }

    pub fn derived(self) -> Holon {

        Holon { 
            uuid: u8, // ?TODO: encode base64
            namespace_id: u8,
            local_id: u8,
            created_at: Timestamp,
            version: SemanticVersion,
            properties: PropertyMap,
            descriptor: HolonDescriptor,
            actions: ActionMap,
            relationships: RelationshipMap
        }
    }
}