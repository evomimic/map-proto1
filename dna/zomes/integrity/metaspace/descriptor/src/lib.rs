
pub trait Descriptor {
    // fn action(params: U) -> ActionResult;
    // fn map_link(&self) -> Relationship;
}

enum Description {
    String
}

enum TimeZone {

}

enum Composite {
    FloatComposite(FloatDescriptor),
    DateTimeComposite(DateTimeDescriptor)
}

enum IntegerFormat {

}

enum StringFormat {

}

struct FloatDescriptor {
    significand: IntegerProperty,
    base: IntegerProperty,
    exponent: IntegerProperty,
    precision: IntegerProperty,
}

struct IntegerDescriptor {
    format: IntegerFormat,
    min_value: u8,
    max_value: u8,
}

struct DateTimeDescriptor {
    date: DateDescriptor,
    time: TimeDescriptor,
    timezone: TimeZone,
}

struct DateDescriptor {
    date: IntegerProperty,
    month: IntegerProperty,
    year: IntegerProperty,
}

struct TimeDescriptor { // meant to be a TimeStamp ?
    seconds: FloatDescriptor,
    hours: IntegerDescriptor,
    
}

struct StringDescriptor {
    min_length: u8,
    max_length: u8,
    pattern: String,
    format: StringFormat
}
impl StringDescriptor {
    fn new(min_length: u8, max_length: u8, pattern: String, format: StringFormat) -> Self {
        Self {
        min_length, 
        max_length,
        pattern, 
        format
        }
    }
}

struct TypeDescriptor {
    uid: u8, // ?TODO: encode base64
    name: String,
    description: String,
    semantic: StringFormat, // IRI
    // created_at: DateTime
}
impl TypeDescriptor {
    fn new(uid: u8, name: String, description: String, semantic: StringFormat) -> Self {
        Self {
        uid, 
        name,
        description, 
        semantic
        }
    }
}


struct HolonDescriptor<T: Descriptor> {
    identifying_properties: Vec<T>,
    properties: Vec<T>,
}
impl<T> HolonDscriptor<T> 
 where 
    T: Descriptor,
{
    fn new( identifying_properties: Vec<T>, properties: T ) -> Self {
        Self {
        identifying_properties,
        properties,
        }
    }
}