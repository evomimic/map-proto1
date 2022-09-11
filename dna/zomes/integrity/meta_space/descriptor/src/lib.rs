
use::hc_zome_coordination_interface::Descriptor;

enum StringFormat {

}

pub struct StringDescriptor {
    min_length: u8,
    max_length: u8,
    pattern: String,
    format: StringFormat
}

impl Descriptor for StringDescriptor {
    fn new(min_length: u8, max_length: u8, pattern: String, format: StringFormat) -> StringDescriptor {
        Self {
            min_length,
            max_length,
            pattern,
            format
        }
    }
}

pub struct TypeDescriptor {
    uid: u8, // TODO: encode base64
    name: String,
    description: String,
    semantic: String, // IRI
    // created_at: DateTime
}

impl Describe for TypeDescriptor {
    fn new(uid: u8, name: String, description: String, semantic: String) -> Self {
        Self {
            uid,
            name,
            description,
            semantic
        }
    }
}

pub struct HolonDescriptor {
    identifying_properties: Box<dyn Descriptor>,
    properties: Box<dyn Descriptor>,
    
}

impl Descriptor for HolonDescriptor {
    fn new<Descriptor>(identifying_properties: Box<dyn Descriptor>, properties: Box<dyn Descriptor>) -> StringDescriptor {
        Self {
            identifying_properties,
            properties,
        }
    }
}