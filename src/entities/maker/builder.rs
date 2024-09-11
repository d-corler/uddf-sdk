use crate::entities::manufacturer::structure::Manufacturer;

use super::structure::Maker;

/// Builder for creating `Maker` instances.
pub struct MakerBuilder {
    manufacturers: Vec<Manufacturer>,
}

impl MakerBuilder {
    /// Create a new `MakerBuilder` instance.
    pub fn new() -> Self {
        MakerBuilder {
            manufacturers: Vec::new(),
        }
    }

    /// Add a manufacturer.
    pub fn add_manufacturer(mut self, manufacturer: Manufacturer) -> Self {
        self.manufacturers.push(manufacturer);
        self
    }

    /// Build the final `Maker` object.
    pub fn build(self) -> Result<Maker, &'static str> {
        Ok(Maker {
            manufacturers: self.manufacturers,
        })
    }
}

impl Default for MakerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
