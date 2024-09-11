use crate::entities::contact::structure::Contact;

use super::structure::Manufacturer;

const MISSING_MANUFACTURER_ID: &str = "Manufacturer ID is required";

/// Builder for creating `Manufacturer` instances.
pub struct ManufacturerBuilder {
    id: Option<String>,
    name: Option<String>,
    contact: Option<Contact>,
}

impl ManufacturerBuilder {
    /// Create a new `ManufacturerBuilder` instance.
    pub fn new() -> Self {
        ManufacturerBuilder {
            id: None,
            name: None,
            contact: None,
        }
    }

    /// Set the ID of the manufacturer.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the contact.
    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    /// Build the final `Manufacturer` object.
    pub fn build(self) -> Result<Manufacturer, &'static str> {
        Ok(Manufacturer {
            id: self.id.ok_or(MISSING_MANUFACTURER_ID)?,
            name: self.name,
            contact: self.contact,
        })
    }
}

impl Default for ManufacturerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
