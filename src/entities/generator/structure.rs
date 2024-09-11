use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::{link::structure::Link, manufacturer::structure::Manufacturer};

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorType {
    Converter,
    DiveComputer,
    LogBook,
}

/// Represents a generator.
///
/// https://www.streit.cc/extern/uddf_v321/en/generator.html
#[derive(Debug, Serialize)]
pub struct Generator {
    pub name: String,
    #[serde(rename = "type")]
    pub generator_type: GeneratorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Manufacturer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "datetime")]
    pub date_time: DateTime<Utc>,
}

impl Generator {
    pub fn validate(&self) -> Result<(), String> {
        if self.manufacturer.is_some() && self.link.is_some() {
            return Err("Generator cannot have both manufacturer and link".to_string());
        }
        Ok(())
    }
}
