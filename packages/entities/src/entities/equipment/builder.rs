use super::structure::{Equipment, Equipment_};

/// Builder for creating `Equipment` instances.
pub struct EquipmentBuilder {
    equipments: Vec<Equipment_>,
}

impl EquipmentBuilder {
    /// Create a new `EquipmentBuilder` instance.
    pub fn new() -> Self {
        EquipmentBuilder {
            equipments: Vec::new(),
        }
    }

    /// Add an equipment.
    pub fn add_equipment(mut self, equipment: Equipment_) -> Self {
        self.equipments.push(equipment);
        self
    }

    /// Build the final `Equipment` object.
    pub fn build(self) -> Result<Equipment, &'static str> {
        Ok(Equipment {
            equipments: self.equipments,
        })
    }
}

impl Default for EquipmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
