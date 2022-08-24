
enum StringFormat {

}

pub struct StringDescriptor {
    min_length: u8,
    max_length: u8,
    pattern: String,
    format: StringFormat
}

pub struct TypeDescriptor {
    uid: u8, // TODO: encode base64
    name: String,
    description: String,
    semantic: String, // IRI
    // created_at: DateTime
}

pub struct ObjectDescriptor {
    
}

pub struct PropertyDescriptor {
    
}