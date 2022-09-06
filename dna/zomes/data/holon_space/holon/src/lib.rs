use hdi::prelude::hash_blake2b;
use hdi::prelude::Timestamp;

enum SemanticVersion {

}

pub struct Holon {
    uid: u8, // TODO: encode base64
    namespace_id: u8,
    local_id: u8,
    created_at: Timestamp,
    version: SemanticVersion
}

impl Holon {
    fn new(namespace_id: u8, local_id: u8, version: SemanticVersion) -> Self {        
        // let holon_id: u8 = hash(&namespace_id + &local_id);
        // let stamp = time.timestamp();
        Self {
            uid: holon_id,
            namespace_id,
            local_id,
            created_at: stamp,
            version,
        }
    };
    
}

// impl Describe for Holon {
//     fn new(&self) -> Self;
    
// }