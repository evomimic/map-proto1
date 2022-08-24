use chrono::{DateTime, Utc};

enum SemanticVersion {

}

pub struct Holon {
    uid: u8, // TODO: encode base64
    namespaceid: u8,
    localid: u8,
    created_at: DateTime<Utc>,
    version: SemanticVersion
}

