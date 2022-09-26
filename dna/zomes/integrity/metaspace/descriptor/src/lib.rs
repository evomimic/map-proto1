
pub trait Descriptor {
    // fn action(params: U) -> ActionResult;
    // fn map_link(&self) -> Relationship;
}

pub enum Description {
    FloatDescription(FloatDescriptor),
    IntegerDescription(IntegerDescriptor),
    DateTimeDescription(DateTimeDescriptor),
    DateDescription(DateDescriptor),
    TimeDescription(TimeDescriptor),
    StringDescription(StringDescriptor),

}

pub struct FloatDescriptor {
    pub significand: IntegerProperty,
    pub base: IntegerProperty,
    pub exponent: IntegerProperty,
    pub precision: IntegerProperty,
}

pub struct IntegerDescriptor {
    pub format: IntegerFormat,
    pub min_value: u8,
    pub max_value: u8,
}

pub struct DateTimeDescriptor {
    pub date: DateDescriptor,
    pub time: TimeDescriptor,
    pub timezone: TimeZone,
}

pub struct DateDescriptor {
    pub date: IntegerProperty,
    pub month: IntegerProperty,
    pub year: IntegerProperty,
}

pub struct TimeDescriptor { // meant to be a TimeStamp ?
    pub seconds: FloatDescriptor,
    pub hours: IntegerDescriptor,
    
}

pub struct StringDescriptor {
    pub min_length: u8,
    pub max_length: u8,
    pub pattern: String,
    pub format: StringFormat
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


pub struct TypeDescriptor {
    pub uid: u8, // ?TODO: encode base64
    pub name: String,
    pub description: Description,
    pub semantic: StringFormat, // IRI
    // pub created_at: DateTime
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


pub struct HolonDescriptor {
    pub properties: PropertyMap,
    pub identifying_properties: HolonDescriptor,
}
impl<T> HolonDescriptor<T> 
//  where 
//     T: Descriptor,
{
    fn new( properties: PropertyMap, identifying_properties: HolonDescriptor ) -> Self {
        Self {
        properties,
        identifying_properties,
        }
    }
}


pub struct RelationshipDescriptor {
    id: String,
    created_at: DateTime,
    //
}