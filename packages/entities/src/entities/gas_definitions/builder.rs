use crate::entities::mix::structure::Mix;

use super::structure::GasDefinitions;

/// Builder for creating `GasDefinitions` instances.
pub struct GasDefinitionsBuilder {
    mixes: Vec<Mix>,
}

impl GasDefinitionsBuilder {
    /// Create a new `GasDefinitionsBuilder` instance.
    pub fn new() -> Self {
        GasDefinitionsBuilder { mixes: Vec::new() }
    }

    /// Add a mix.
    pub fn add_mix(mut self, mix: Mix) -> Self {
        self.mixes.push(mix);
        self
    }

    /// Build the final `GasDefinitions` object.
    pub fn build(self) -> Result<GasDefinitions, &'static str> {
        Ok(GasDefinitions { mixes: self.mixes })
    }
}

impl Default for GasDefinitionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
