use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Certificate {
    pub _id: u32,
    name: String,
    course: String,
}
