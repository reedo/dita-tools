use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "grammar")]
pub struct Grammar {
    #[serde(rename = "@ns")]
    pub ns: String,
    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: String,
    // pub other_attrs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Start {}

#[derive(Serialize, Deserialize)]
pub struct Define {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ref {}
