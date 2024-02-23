use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Certificate {
    #[serde(rename = "_id")]
    pub id: u32,
    name: String,
    course: String,
}
