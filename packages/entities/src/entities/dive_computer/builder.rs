use crate::entities::{link::structure::Link, manufacturer::structure::Manufacturer};

use super::structure::DiveComputer;

const MISSING_DIVE_COMPUTER_ID: &str = "Dive computer ID is required";
const MISSING_DIVE_COMPUTER_MANUFACTURER_OR_LINK: &str =
    "Dive computer manufacturer or link is required";

const CONFLICTING_DIVE_COMPUTER_MANUFACTURER_AND_LINK: &str =
    "Dive computer cannot have both manufacturer and link";

/// Builder for creating `DiveComputer` instances.
pub struct DiveComputerBuilder {
    id: Option<String>,
    name: Option<String>,
    model: Option<String>,
    manufacturer: Option<Manufacturer>,
    link: Option<Link>,
    serial_number: Option<String>,
}

impl DiveComputerBuilder {
    /// Create a new `DiveComputerBuilder` instance.
    pub fn new() -> Self {
        DiveComputerBuilder {
            id: None,
            name: None,
            model: None,
            manufacturer: None,
            link: None,
            serial_number: None,
        }
    }

    /// Set the ID of the dive computer.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the model.
    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
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

    /// Set the serial number.
    pub fn serial_number(mut self, serial_number: String) -> Self {
        self.serial_number = Some(serial_number);
        self
    }

    /// Build the final `DiveComputer` object.
    pub fn build(self) -> Result<DiveComputer, &'static str> {
        if self.manufacturer.is_some() && self.link.is_some() {
            return Err(CONFLICTING_DIVE_COMPUTER_MANUFACTURER_AND_LINK);
        }

        if self.manufacturer.is_none() && self.link.is_none() {
            return Err(MISSING_DIVE_COMPUTER_MANUFACTURER_OR_LINK);
        }

        Ok(DiveComputer {
            id: self.id.ok_or(MISSING_DIVE_COMPUTER_ID)?,
            name: self.name,
            model: self.model,
            manufacturer: self.manufacturer,
            link: self.link,
            serial_number: self.serial_number,
        })
    }
}

impl Default for DiveComputerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
