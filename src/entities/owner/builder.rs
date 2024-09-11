use crate::entities::{equipment::structure::Equipment, personal::structure::Personal};

use super::structure::Owner;

const MISSING_OWNER_ID: &str = "Owner ID is required";

/// Builder for creating `Owner` instances.
pub struct OwnerBuilder {
    id: Option<String>,
    personal: Option<Personal>,
    equipment: Option<Equipment>,
}

impl OwnerBuilder {
    /// Create a new `OwnerBuilder` instance.
    pub fn new() -> Self {
        OwnerBuilder {
            id: None,
            personal: None,
            equipment: None,
        }
    }

    /// Set the ID of the owner.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the personal.
    pub fn personal(mut self, personal: Personal) -> Self {
        self.personal = Some(personal);
        self
    }

    /// Set the equipment.
    pub fn equipment(mut self, equipment: Equipment) -> Self {
        self.equipment = Some(equipment);
        self
    }

    /// Build the final `Owner` object.
    pub fn build(self) -> Result<Owner, &'static str> {
        Ok(Owner {
            id: self.id.ok_or(MISSING_OWNER_ID)?,
            personal: self.personal,
            equipment: self.equipment,
        })
    }
}

impl Default for OwnerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
