use bincode::{Decode, Encode};
use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Uid;

impl Uid {
    pub fn new(templ: &str) -> String {
        // It was easy to use a UUID directly, but it would be too long for output. To prevent this,
        // we used the first two characters of the template string (e.g., Register, Function, Table, Entry)
        // and the first eight characters of the UUID.
        let uid = uuid::Uuid::new_v4().to_string();
        format!("{}-{}", &templ[0..2], uid[0..8].to_string())
    }
}
