use serde::Serialize;

use crate::entities::{link::structure::Link, manufacturer::structure::Manufacturer};

/// Represents a dive computer.
///
/// https://www.streit.cc/extern/uddf_v321/en/divecomputer.html
#[derive(Debug, Serialize)]
pub struct DiveComputer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Manufacturer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "serialnumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
