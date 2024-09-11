use chrono::{DateTime, Utc};

use crate::entities::{link::structure::Link, manufacturer::structure::Manufacturer};

use super::structure::{Generator, GeneratorType};

const MISSING_GENERATOR_NAME: &str = "Generator name is required";
const MISSING_GENERATOR_TYPE: &str = "Generator type is required";
const MISSING_GENERATOR_MANUFACTURER_OR_LINK: &str = "Generator manufacturer or link is required";
const MISSING_GENERATOR_DATE_TIME: &str = "Generator date time is required";

const CONFLICTING_GENERATOR_MANUFACTURER_AND_LINK: &str =
    "Generator cannot have both manufacturer and link";

/// Builder for creating `Generator` instances.
pub struct GeneratorBuilder {
    name: Option<String>,
    generator_type: Option<GeneratorType>,
    manufacturer: Option<Manufacturer>,
    link: Option<Link>,
    date_time: Option<DateTime<Utc>>,
}

impl GeneratorBuilder {
    /// Create a new `GeneratorBuilder` instance.
    pub fn new() -> Self {
        GeneratorBuilder {
            name: None,
            generator_type: None,
            manufacturer: None,
            link: None,
            date_time: None,
        }
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the type.
    pub fn generator_type(mut self, generator_type: GeneratorType) -> Self {
        self.generator_type = Some(generator_type);
        self
    }

    /// Set the manufacturer.
    pub fn manufacturer(mut self, manufacturer: Manufacturer) -> Self {
        self.manufacturer = Some(manufacturer);
        self
    }

    /// Set the link.
    pub fn link(mut self, link: Link) -> Self {
        self.link = Some(link);
        self
    }

    /// Set the date time.
    pub fn date_time(mut self, date_time: DateTime<Utc>) -> Self {
        self.date_time = Some(date_time);
        self
    }

    /// Build the final `Generator` object.
    pub fn build(self) -> Result<Generator, &'static str> {
        if self.manufacturer.is_some() && self.link.is_some() {
            return Err(CONFLICTING_GENERATOR_MANUFACTURER_AND_LINK);
        }

        if self.manufacturer.is_none() && self.link.is_none() {
            return Err(MISSING_GENERATOR_MANUFACTURER_OR_LINK);
        }

        Ok(Generator {
            name: self.name.ok_or(MISSING_GENERATOR_NAME)?,
            generator_type: self.generator_type.ok_or(MISSING_GENERATOR_TYPE)?,
            manufacturer: self.manufacturer,
            link: self.link,
            date_time: self.date_time.ok_or(MISSING_GENERATOR_DATE_TIME)?,
        })
    }
}

impl Default for GeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
