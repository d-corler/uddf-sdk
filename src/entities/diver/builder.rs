use crate::entities::owner::structure::Owner;

use super::structure::Diver;

/// Builder for creating `Diver` instances.
pub struct DiverBuilder {
    owner: Option<Owner>,
}

impl DiverBuilder {
    /// Create a new `DiverBuilder` instance.
    pub fn new() -> Self {
        DiverBuilder { owner: None }
    }

    /// Set the owner.
    pub fn owner(mut self, owner: Owner) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Build the final `Diver` object.
    pub fn build(self) -> Result<Diver, &'static str> {
        Ok(Diver { owner: self.owner })
    }
}

impl Default for DiverBuilder {
    fn default() -> Self {
        Self::new()
    }
}
