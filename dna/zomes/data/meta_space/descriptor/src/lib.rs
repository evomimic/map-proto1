
use::hc_zome_coordination_interface::Describe;
enum StringFormat {

}

pub struct StringDescriptor {
    min_length: u8,
    max_length: u8,
    pattern: String,
    format: StringFormat
}

impl Describe for StringDescriptor {
    fn new(min_length: u8, max_length: u8, pattern: String, format: StringFormat) -> StringDescriptor {
        Self {
            min_length,
            max_length,
            pattern,
            format
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
    fn new(&self) -> Self;
    
}
pub struct ObjectDescriptor {
    
}

impl Describe for ObjectDescriptor {
    fn new(&self) -> Self;
    
}

pub struct PropertyDescriptor {
    
}

impl Describe for PropertyDescriptor {
    fn new(&self) -> Self;
    
}