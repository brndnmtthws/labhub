use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PushEvent {
    name: String,
    age: u8,
    phones: Vec<String>,
}
